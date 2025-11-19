use std::{
    env::var,
    fs::{read_to_string, write},
    path::Path,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use {
    base64::{Engine, engine::general_purpose::STANDARD},
    dotenvy::from_path,
    md5::compute,
    regex::Regex,
    reqwest::{Client, Response, get},
    serde::de::DeserializeOwned,
    serde_json::from_str,
    url::form_urlencoded::byte_serialize,
};

use crate::errors::QobuzApiError::{
    self, ApiResponseParseError, DownloadError, HttpError, QobuzApiInitializationError,
};

/// Computes the MD5 hash of the input string.
///
/// This function takes a string slice and returns its MD5 hash as a hexadecimal string.
/// MD5 hashing is commonly used for generating unique identifiers or for basic data
/// integrity verification.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input to be hashed
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the MD5 hash
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::utils::get_md5_hash;
///
/// let hash = get_md5_hash("hello world");
/// assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
/// ```
pub fn get_md5_hash(input: &str) -> String {
    format!("{:x}", compute(input.as_bytes()))
}

/// Builds a query string from a collection of key-value pairs.
///
/// This function takes a slice of tuples containing string keys and values, filters out
/// any pairs with empty values, URL-encodes the keys and values, and joins them with
/// ampersands to form a valid query string. This is commonly used when constructing
/// API requests that require query parameters.
///
/// # Arguments
///
/// * `params` - A slice of tuples containing key-value pairs as strings
///
/// # Returns
///
/// A `String` containing the URL-encoded query string
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::utils::to_query_string;
///
/// let params = vec![
///     ("name".to_string(), "John".to_string()),
///     ("age".to_string(), "30".to_string()),
///     ("city".to_string(), "".to_string()), // This will be filtered out
/// ];
/// let query_string = to_query_string(&params);
/// assert_eq!(query_string, "name=John&age=30");
/// ```
pub fn to_query_string(params: &[(String, String)]) -> String {
    let filtered_params: Vec<String> = params
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| {
            byte_serialize(key.as_bytes()).collect::<String>()
                + "="
                + &byte_serialize(value.as_bytes()).collect::<String>()
        })
        .collect();

    filtered_params.join("&")
}

/// Gets the current Unix timestamp as a string.
///
/// This function returns the current time as a Unix timestamp (number of seconds
/// since January 1, 1970 UTC) formatted as a string. Unix timestamps are commonly
/// used in API requests that require time-based parameters or for generating
/// unique identifiers based on time.
///
/// # Returns
///
/// A `String` containing the current Unix timestamp
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::utils::get_current_timestamp;
/// use std::thread::sleep;
/// use std::time::Duration;
///
/// let timestamp1 = get_current_timestamp();
/// sleep(Duration::from_millis(1000)); // Sleep for 1 second
/// let timestamp2 = get_current_timestamp();
/// // The timestamps should be different (or the same if called in the same second)
/// ```
pub fn get_current_timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        .to_string()
}

/// Extracts the app ID from Qobuz Web Player's bundle.js file.
///
/// This asynchronous function fetches the Qobuz Web Player's JavaScript bundle file
/// and extracts the application ID using regular expressions. The app ID is required
/// for authenticating with the Qobuz API. This function is useful when you don't have
/// a pre-configured app ID and need to extract it dynamically from the web player.
///
/// # Returns
///
/// * `Ok(String)` - The extracted app ID if found in the bundle
/// * `Err(Box<dyn Error>)` - If the bundle couldn't be fetched or the app ID couldn't be extracted
///
/// # Errors
///
/// This function will return an error if:
/// - The web request to fetch the bundle.js fails
/// - The regular expression pattern fails to match
/// - The app ID cannot be extracted from the bundle content
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::get_web_player_app_id;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let app_id = get_web_player_app_id().await?;
///     println!("App ID: {}", app_id);
///     Ok(())
/// }
/// ```
pub async fn get_web_player_app_id() -> Result<String, QobuzApiError> {
    let bundle_content = fetch_bundle_js().await?;

    // Extract app_id from bundle.js using regex
    let re =
        Regex::new(r#"production:\{api:\{appId:"(?P<appID>[^"]*)",appSecret:"#).map_err(|e| {
            QobuzApiInitializationError {
                message: format!("Failed to create regex for app ID extraction: {}", e),
            }
        })?;
    if let Some(caps) = re.captures(&bundle_content)
        && let Some(app_id) = caps.name("appID")
    {
        return Ok(app_id.as_str().to_string());
    }

    Err(QobuzApiInitializationError {
        message: "Failed to extract app_id from bundle.js".to_string(),
    })
}

/// Extracts the app secret from Qobuz Web Player's bundle.js file.
///
/// This asynchronous function fetches the Qobuz Web Player's JavaScript bundle file
/// and extracts the application secret using a complex multi-step process involving
/// regular expressions and base64 decoding. The app secret is required for
/// authenticating with the Qobuz API. This function is useful when you don't have
/// a pre-configured app secret and need to extract it dynamically from the web player.
///
/// The extraction process involves:
/// 1. Finding seed and timezone information in the bundle
/// 2. Processing timezone information to find relevant sections
/// 3. Extracting info and extras data
/// 4. Combining and truncating the data
/// 5. Base64 decoding the result to get the app secret
///
/// # Returns
///
/// * `Ok(String)` - The extracted app secret if found in the bundle
/// * `Err(Box<dyn Error>)` - If the bundle couldn't be fetched or the app secret couldn't be extracted
///
/// # Errors
///
/// This function will return an error if:
/// - The web request to fetch the bundle.js fails
/// - Any of the regular expression patterns fail to match
/// - The concatenated string is too short for processing
/// - Base64 decoding fails
/// - UTF-8 conversion of the decoded bytes fails
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::get_web_player_app_secret;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let app_secret = get_web_player_app_secret().await?;
///     println!("App Secret: {}", app_secret);
///     Ok(())
/// }
/// ```
pub async fn get_web_player_app_secret() -> Result<String, QobuzApiError> {
    let bundle_content = fetch_bundle_js().await?;

    // Extract seed and timezone from bundle.js
    let seed_timezone_re = Regex::new(
        r#"\):[a-z]\.initialSeed\("(?P<seed>.*?)",window\.utimezone\.(?P<timezone>[a-z]+)\)"#,
    )
    .map_err(|e| QobuzApiInitializationError {
        message: format!("Failed to create regex for seed/timezone extraction: {}", e),
    })?;
    let seed_timezone_caps =
        seed_timezone_re
            .captures(&bundle_content)
            .ok_or(QobuzApiInitializationError {
                message: "Failed to find seed and timezone in bundle.js".to_string(),
            })?;

    let seed = seed_timezone_caps
        .name("seed")
        .map(|m| m.as_str())
        .unwrap_or("");
    let timezone = seed_timezone_caps
        .name("timezone")
        .map(|m| m.as_str())
        .unwrap_or("");
    let title_case_timezone = capitalize_first_letter(timezone);

    // Extract info and extras for the production timezone
    let info_extras_pattern = format!(r#"name:"[^"]*/{}"[^}}]*"#, title_case_timezone);
    let info_extras_re =
        Regex::new(&info_extras_pattern).map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to create regex for info/extras extraction: {}", e),
        })?;
    let info_extras_caps =
        info_extras_re
            .captures(&bundle_content)
            .ok_or(QobuzApiInitializationError {
                message: "Failed to find info and extras in bundle.js".to_string(),
            })?;

    let timezone_object_str = info_extras_caps.get(0).map_or("", |m| m.as_str());

    let info_re =
        Regex::new(r#"info:"(?P<info>[^"]*)""#).map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to create regex for info extraction: {}", e),
        })?;
    let info = info_re
        .captures(timezone_object_str)
        .and_then(|c| c.name("info"))
        .map_or("", |m| m.as_str());

    let extras_re =
        Regex::new(r#"extras:"(?P<extras>[^"]*)""#).map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to create regex for extras extraction: {}", e),
        })?;
    let extras = extras_re
        .captures(timezone_object_str)
        .and_then(|c| c.name("extras"))
        .map_or("", |m| m.as_str());

    // Concatenate seed, info, and extras, then remove last 44 characters
    let mut base64_encoded_secret = format!("{}{}{}", seed, info, extras);
    if base64_encoded_secret.len() > 44 {
        base64_encoded_secret.truncate(base64_encoded_secret.len() - 44);
    } else {
        return Err(QobuzApiInitializationError {
            message: "Concatenated string is too short".to_string(),
        });
    }

    // Decode base64 to get the app secret
    let decoded_bytes =
        STANDARD
            .decode(base64_encoded_secret)
            .map_err(|e| QobuzApiInitializationError {
                message: format!("Failed to decode base64 encoded secret: {}", e),
            })?;
    let app_secret = String::from_utf8(decoded_bytes).map_err(|e| QobuzApiInitializationError {
        message: format!("Failed to convert decoded bytes to string: {}", e),
    })?;

    Ok(app_secret)
}

/// Helper function to fetch bundle.js content from Qobuz Web Player.
///
/// This internal asynchronous function retrieves the JavaScript bundle file from
/// the Qobuz Web Player by first fetching the login page to find the bundle URL,
/// then downloading the actual bundle file. This is used by other functions to
/// extract API credentials from the web player.
///
/// # Returns
///
/// * `Ok(String)` - The content of the bundle.js file if successfully fetched
/// * `Err(Box<dyn Error>)` - If the web requests fail or the bundle URL cannot be found
///
/// # Errors
///
/// This function will return an error if:
/// - The request to the login page fails
/// - The bundle.js URL cannot be found in the login page
/// - The request to the bundle.js file fails
/// - The response cannot be converted to text
async fn fetch_bundle_js() -> Result<String, QobuzApiError> {
    let client = Client::new();

    // Get the login page to find the bundle.js URL
    let login_page = client
        .get("https://play.qobuz.com/login")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/110.0",
        )
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to fetch login page: {}", e),
        })?
        .text()
        .await
        .map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to read login page content: {}", e),
        })?;

    // Extract the bundle.js URL from the HTML
    let bundle_js_re =
        Regex::new(r#"<script src="(?P<bundleJS>/resources/\d+\.\d+\.\d+-[a-z]\d{3}/bundle\.js)"#)
            .map_err(|e| QobuzApiInitializationError {
                message: format!("Failed to create regex for bundle.js URL extraction: {}", e),
            })?;
    let bundle_js_match =
        bundle_js_re
            .captures(&login_page)
            .ok_or(QobuzApiInitializationError {
                message: "Failed to find bundle.js URL in login page".to_string(),
            })?;

    let bundle_js_suffix = bundle_js_match
        .name("bundleJS")
        .ok_or(QobuzApiInitializationError {
            message: "Failed to extract bundle.js suffix".to_string(),
        })?
        .as_str();

    // Fetch the actual bundle.js content
    let bundle_url = format!("https://play.qobuz.com{}", bundle_js_suffix);
    let bundle_content = client
        .get(&bundle_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/110.0",
        )
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to fetch bundle.js: {}", e),
        })?
        .text()
        .await
        .map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to read bundle.js content: {}", e),
        })?;

    Ok(bundle_content)
}

/// Helper function to capitalize the first letter of a string.
///
/// This internal function takes a string and returns a new string with the first
/// character converted to uppercase while leaving the rest of the string unchanged.
/// This is used in the app secret extraction process to properly format timezone names.
///
/// # Arguments
///
/// * `s` - A string slice to capitalize
///
/// # Returns
///
/// A `String` with the first character capitalized (if any)
///
/// # Examples
///
/// ```
/// # use qobuz_api_rust::utils::capitalize_first_letter;
/// #
/// assert_eq!(capitalize_first_letter("hello"), "Hello");
/// assert_eq!(capitalize_first_letter("world"), "World");
/// assert_eq!(capitalize_first_letter(""), "");
/// ```
pub fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Sanitizes a string to be used as a filename by removing or replacing invalid characters.
///
/// This function takes a filename string and sanitizes it by replacing characters
/// that are invalid in filenames across different operating systems. It also trims
/// leading/trailing spaces and periods, and limits the length to prevent filesystem issues.
/// This is particularly useful when saving files with user-provided names or names
/// derived from API responses.
///
/// # Arguments
///
/// * `filename` - A string slice containing the filename to sanitize
///
/// # Returns
///
/// A `String` containing the sanitized filename
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::utils::sanitize_filename;
///
/// assert_eq!(sanitize_filename("valid_filename.txt"), "valid_filename.txt");
/// assert_eq!(sanitize_filename("invalid<char>.txt"), "invalid_char_.txt");
/// assert_eq!(sanitize_filename("  spaced name  "), "spaced name");
/// ```
pub fn sanitize_filename(filename: &str) -> String {
    // Replace invalid characters for filenames with safe alternatives
    // Windows and Unix systems have different restrictions, so we use the more restrictive set
    let mut sanitized = filename
        .replace(
            |c: char| {
                c == '<' || c == '>' || c == ':' || c == '"' || c == '|' || c == '?' || c == '*'
            },
            "_",
        )
        .replace(['/', '\\', '\0'], "_"); // null character

    // Remove leading/trailing spaces and periods that may cause issues
    sanitized = sanitized
        .trim()
        .trim_start_matches('.')
        .trim_end_matches('.')
        .to_string();

    // Limit length to avoid filesystem issues (most filesystems support up to 255 bytes)
    if sanitized.len() > 200 {
        sanitized.truncate(200);
        // Ensure we don't end up with a trailing space or period after truncation
        sanitized = sanitized.trim_end().to_string();
    }

    sanitized
}

/// Deserializes an HTTP response to the expected type.
///
/// This asynchronous function reads the text content from an HTTP response and
/// attempts to deserialize it into the specified type using serde. This is a
/// utility function used throughout the library to convert API responses into
/// Rust data structures. It handles both the reading of the response body and
/// the deserialization process, providing appropriate error handling for both steps.
///
/// # Type Parameters
///
/// * `T` - The type to deserialize the response into, must implement `DeserializeOwned`
///
/// # Arguments
///
/// * `response` - The HTTP response to deserialize
///
/// # Returns
///
/// * `Ok(T)` - The deserialized data if successful
/// * `Err(QobuzApiError)` - If reading the response or deserializing fails
///
/// # Errors
///
/// This function will return an error if:
/// - Reading the response body fails
/// - Deserializing the response body to the target type fails
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::deserialize_response;
/// use serde_json::Value;
/// use reqwest::get;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let response = get("https://httpbin.org/json").await.map_err(qobuz_api_rust::QobuzApiError::HttpError)?;
///     let data: Value = deserialize_response(response).await?;
///     println!("{:?}", data);
///     Ok(())
/// }
/// ```
pub async fn deserialize_response<T>(response: Response) -> Result<T, QobuzApiError>
where
    T: DeserializeOwned,
{
    let content = response.text().await.map_err(HttpError)?;

    // Check if the response is empty, which might indicate an issue
    if content.trim().is_empty() {
        return Err(QobuzApiInitializationError {
            message: "Received empty response from API".to_string(),
        });
    }

    from_str::<T>(&content).map_err(|source| ApiResponseParseError {
        content: content.clone(),
        source,
    })
}

/// Reads app credentials from a .env file.
///
/// This function attempts to read Qobuz API credentials (app ID and app secret)
/// from environment variables, loading them from a .env file if it exists.
/// The credentials are expected to be stored in environment variables named
/// `QOBUZ_APP_ID` and `QOBUZ_APP_SECRET`. This function is useful for initializing
/// the Qobuz API service with stored credentials.
///
/// # Returns
///
/// * `Ok((Option<String>, Option<String>))` - A tuple containing the app ID and app secret,
///   with `None` for each if not found in environment variables
/// * `Err(Box<dyn Error>)` - If there's an issue reading the .env file
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::read_app_credentials_from_env;
///
/// match read_app_credentials_from_env() {
///     Ok((Some(app_id), Some(app_secret))) => {
///         println!("Found credentials: {}, {}", app_id, app_secret);
///     }
///     Ok((None, None)) => {
///         println!("No credentials found in environment");
///     }
///     Ok((Some(app_id), None)) => {
///         println!("Found app ID but no app secret: {}", app_id);
///     }
///     Ok((None, Some(_))) => {
///         println!("Found app secret but no app ID");
///     }
///     Err(e) => {
///         eprintln!("Error reading credentials: {}", e);
///     }
/// }
/// ```
pub fn read_app_credentials_from_env() -> Result<(Option<String>, Option<String>), QobuzApiError> {
    // Try to load from .env file
    if Path::new(".env").exists()
        && let Err(e) = from_path(".env")
    {
        eprintln!("Warning: Failed to load .env file: {}", e);
    }

    let app_id = var("QOBUZ_APP_ID").ok();
    let app_secret = var("QOBUZ_APP_SECRET").ok();

    Ok((app_id, app_secret))
}

/// Writes app credentials to a .env file.
///
/// This function saves Qobuz API credentials (app ID and app secret) to a .env file.
/// If the file already exists, it updates the existing entries; otherwise, it creates
/// a new file. The credentials are stored in environment variables named
/// `QOBUZ_APP_ID` and `QOBUZ_APP_SECRET`. This function is useful for caching
/// credentials retrieved from the web player for future use.
///
/// # Arguments
///
/// * `app_id` - The app ID to save
/// * `app_secret` - The app secret to save
///
/// # Returns
///
/// * `Ok(())` - If the credentials were successfully written to the file
/// * `Err(Box<dyn Error>)` - If there's an issue reading or writing the .env file
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::write_app_credentials_to_env;
///
/// # async fn example() -> Result<(), qobuz_api_rust::QobuzApiError> {
/// let result = write_app_credentials_to_env("my_app_id", "my_app_secret");
/// match result {
///     Ok(()) => println!("Credentials saved successfully"),
///     Err(e) => eprintln!("Error saving credentials: {}", e),
/// }
/// # Ok(())
/// # }
/// ```
pub fn write_app_credentials_to_env(app_id: &str, app_secret: &str) -> Result<(), QobuzApiError> {
    // Read existing content or start with empty string
    let env_content = if Path::new(".env").exists() {
        read_to_string(".env").map_err(|e| QobuzApiInitializationError {
            message: format!("Failed to read .env file: {}", e),
        })?
    } else {
        String::new()
    };

    // Parse existing content to avoid duplicating entries
    let mut lines: Vec<String> = env_content.lines().map(|s| s.to_string()).collect();
    let mut app_id_found = false;
    let mut app_secret_found = false;

    for line in &mut lines {
        if line.starts_with("QOBUZ_APP_ID=") {
            *line = format!("QOBUZ_APP_ID={}", app_id);
            app_id_found = true;
        } else if line.starts_with("QOBUZ_APP_SECRET=") {
            *line = format!("QOBUZ_APP_SECRET={}", app_secret);
            app_secret_found = true;
        }
    }

    // Add missing entries
    if !app_id_found {
        lines.push(format!("QOBUZ_APP_ID={}", app_id));
    }
    if !app_secret_found {
        lines.push(format!("QOBUZ_APP_SECRET={}", app_secret));
    }

    // Write back to .env file
    write(".env", lines.join("\n")).map_err(|e| QobuzApiInitializationError {
        message: format!("Failed to write to .env file: {}", e),
    })?;

    Ok(())
}

/// Downloads an image from a URL asynchronously.
///
/// This function retrieves an image from the specified URL and returns the
/// image data as a vector of bytes. It's commonly used to download album art,
/// artist images, or other media associated with Qobuz content. The function
/// checks the HTTP response status and returns an error if the request fails.
///
/// # Arguments
///
/// * `url` - A string slice containing the URL of the image to download
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The image data as a vector of bytes if the download is successful
/// * `Err(Box<dyn Error>)` - If the HTTP request fails or the response status is not successful
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request fails
/// - The response status is not a success (2xx status code)
/// - Reading the response body fails
///
/// # Examples
///
/// ```no_run
/// use qobuz_api_rust::utils::download_image;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let image_data = download_image("https://example.com/image.jpg").await?;
///     println!("Downloaded {} bytes", image_data.len());
///     Ok(())
/// }
/// ```
pub async fn download_image(url: &str) -> Result<Vec<u8>, QobuzApiError> {
    let response = get(url).await.map_err(HttpError)?;
    if !response.status().is_success() {
        return Err(DownloadError {
            message: format!("Failed to download image: HTTP {}", response.status()),
        });
    }
    let bytes = response.bytes().await.map_err(HttpError)?;
    Ok(bytes.to_vec())
}

/// Converts a Unix timestamp to a "YYYY-MM-DD" string and extracts the year.
///
/// This function provides a basic conversion from a Unix timestamp (seconds since epoch)
/// to a formatted date string ("YYYY-MM-DD") and the corresponding year.
/// This implementation is a simplified version and does not account for timezones
/// or complex calendar rules (like leap seconds, historical calendar changes).
/// It assumes the timestamp is in UTC and performs a basic calculation to derive
/// the date components.
///
/// # Arguments
///
/// * `timestamp` - The Unix timestamp (seconds since January 1, 1970 UTC)
///
/// # Returns
///
/// A tuple containing:
/// - `Option<String>`: The formatted date string "YYYY-MM-DD", or `None` if the conversion fails.
/// - `Option<u32>`: The year as a `u32`, or `None` if the conversion fails.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::utils::timestamp_to_date_and_year;
///
/// // Example timestamp for 2023-10-27 10:00:00 UTC
/// let timestamp = 1698393600;
/// let (date_str, year) = timestamp_to_date_and_year(timestamp);
/// assert_eq!(date_str, Some("2023-10-27".to_string()));
/// assert_eq!(year, Some(2023));
/// ```
pub fn timestamp_to_date_and_year(timestamp: i64) -> (Option<String>, Option<u32>) {
    // Number of seconds in a day
    const SECONDS_PER_DAY: i64 = 86_400;

    // Unix epoch starts on January 1, 1970
    let mut days_since_epoch = timestamp / SECONDS_PER_DAY;
    let mut year = 1970;

    // Determine the year
    loop {
        let is_leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_current_year = if is_leap_year { 366 } else { 365 };

        if days_since_epoch < days_in_current_year {
            break; // Found the correct year
        }

        days_since_epoch -= days_in_current_year;
        year += 1;
    }

    // Now days_since_epoch holds the day of the year (0-indexed)
    // Month lengths (non-leap year)
    let month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let month_lengths_leap = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let is_current_year_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let current_month_lengths = if is_current_year_leap {
        &month_lengths_leap
    } else {
        &month_lengths
    };

    let mut month = 1;
    let mut day = 0;
    let mut days_in_months_passed = 0;

    for (i, &len) in current_month_lengths.iter().enumerate() {
        if days_since_epoch < (days_in_months_passed + len as i64) {
            month = i + 1;
            day = (days_since_epoch - days_in_months_passed) + 1;
            break;
        }
        days_in_months_passed += len as i64;
    }

    if day == 0 {
        // Fallback or error case if day calculation fails
        (None, None)
    } else {
        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);
        (Some(date_str), Some(year as u32))
    }
}

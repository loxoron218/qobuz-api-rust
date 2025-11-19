use thiserror::Error;

/// Custom error types for the Qobuz API Rust library.
///
/// This enum represents all possible errors that can occur when using the Qobuz API library.
/// It includes errors from API responses, network operations, parsing, authentication, and more.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::QobuzApiError;
///
/// fn handle_error(error: QobuzApiError) {
///     match error {
///         QobuzApiError::ApiErrorResponse { code, message, status } => {
///             eprintln!("API Error: {} - {} (Status: {})", code, message, status);
///         }
///         QobuzApiError::HttpError(e) => {
///             eprintln!("HTTP Error: {}", e);
///         }
///         _ => eprintln!("Other error occurred: {:?}", error),
///     }
/// }
/// ```
#[derive(Error, Debug)]
pub enum QobuzApiError {
    /// Error response from the Qobuz API.
    ///
    /// This variant represents an error response received from the Qobuz API itself,
    /// containing the error code, message, and status returned by the API.
    ///
    /// # Fields
    ///
    /// * `code` - The error code returned by the API
    /// * `message` - The error message provided by the API
    /// * `status` - The status string returned by the API
    #[error("API Error - Code: {code}, Message: {message}, Status: {status}")]
    ApiErrorResponse {
        /// The error code returned by the API
        code: String,
        /// The error message provided by the API
        message: String,
        /// The status string returned by the API
        status: String,
    },

    /// Error when parsing API response.
    ///
    /// This variant represents an error that occurs when attempting to parse the
    /// response from the Qobuz API into a Rust data structure. It includes the
    /// original content that failed to parse and the underlying parsing error.
    ///
    /// # Fields
    ///
    /// * `content` - The raw content that failed to parse
    /// * `source` - The underlying parsing error from `serde_json`
    #[error("Failed to parse API response: {source}")]
    ApiResponseParseError {
        /// The raw content that failed to parse
        content: String,
        /// The underlying parsing error from `serde_json`
        #[source]
        source: serde_json::Error,
    },

    /// Error during API initialization.
    ///
    /// This variant represents an error that occurs during the initialization
    /// of the Qobuz API service, such as when failing to extract credentials
    /// from the web player or when required configuration is missing.
    ///
    /// # Fields
    ///
    /// * `message` - A description of the initialization error
    #[error("Qobuz API initialization error: {message}")]
    QobuzApiInitializationError {
        /// A description of the initialization error
        message: String,
    },

    /// HTTP request error.
    ///
    /// This variant wraps errors from the `reqwest` crate that occur during
    /// HTTP communication with the Qobuz API. This includes connection errors,
    /// timeout errors, and other network-related issues.
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Network/IO error.
    ///
    /// This variant wraps errors from the standard library's `std::io::Error`
    /// that occur during network or file I/O operations, such as when downloading
    /// tracks or reading local files.
    #[error("Network/IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Lofty library error.
    ///
    /// This variant wraps errors from the `lofty` crate that occur during
    /// audio file metadata operations, such as reading, writing, or modifying tags.
    /// This includes errors when reading from or saving to audio files.
    #[error("Lofty metadata error: {0}")]
    LoftyError(#[from] lofty::error::LoftyError),

    /// URL parsing error.
    ///
    /// This variant wraps errors from the `url` crate that occur when parsing
    /// or constructing URLs for API endpoints.
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    /// Authentication error.
    ///
    /// This variant represents an error that occurs during authentication
    /// with the Qobuz API, such as invalid credentials or expired tokens.
    ///
    /// # Fields
    ///
    /// * `message` - A description of the authentication error
    #[error("Authentication error: {message}")]
    AuthenticationError {
        /// A description of the authentication error
        message: String,
    },

    /// Error when credentials are missing or invalid.
    ///
    /// This variant represents an error that occurs when required credentials
    /// (app ID, app secret, user token, etc.) are missing, empty, or invalid.
    #[error("Missing or invalid credentials: {message}")]
    CredentialsError {
        /// A description of the credential issue
        message: String,
    },

    /// Error when downloading content.
    ///
    /// This variant represents an error that occurs during content download operations,
    /// such as when downloading tracks, images, or other media files.
    #[error("Download error: {message}")]
    DownloadError {
        /// A description of the download issue
        message: String,
    },

    /// Error when processing metadata.
    ///
    /// This variant represents an error that occurs during metadata extraction,
    /// embedding, or processing operations.
    #[error("Metadata processing error: {source}")]
    MetadataError {
        /// The underlying error from metadata operations
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Error when a required resource is not found.
    ///
    /// This variant represents an error that occurs when a requested resource
    /// (track, album, artist, etc.) is not found in the Qobuz API.
    #[error("Resource not found: {resource_type} with ID {resource_id}")]
    ResourceNotFoundError {
        /// The type of resource that was not found
        resource_type: String,
        /// The ID of the resource that was not found
        resource_id: String,
    },

    /// Error when a rate limit is exceeded.
    ///
    /// This variant represents an error that occurs when the Qobuz API rate limit
    /// is exceeded, typically resulting in a 429 HTTP status code.
    #[error("Rate limit exceeded: {message}")]
    RateLimitError {
        /// A description of the rate limit issue
        message: String,
    },

    /// Error when an invalid parameter is provided to an API call.
    ///
    /// This variant represents an error that occurs when invalid or unsupported
    /// parameters are passed to an API endpoint.
    #[error("Invalid parameter: {message}")]
    InvalidParameterError {
        /// A description of the invalid parameter
        message: String,
    },

    /// Error when the API returns an unexpected response format.
    ///
    /// This variant represents an error that occurs when the API returns a response
    /// that doesn't match the expected format, indicating a potential API change or bug.
    #[error("Unexpected API response format: {message}")]
    UnexpectedApiResponseError {
        /// A description of the unexpected response
        message: String,
    },
}

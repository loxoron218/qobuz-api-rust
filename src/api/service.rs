use std::{env::var, iter::once};

use reqwest::{
    Client,
    header::{HeaderName, HeaderValue},
};

use crate::{
    errors::QobuzApiError::{
        self, AuthenticationError, CredentialsError, HttpError, QobuzApiInitializationError,
    },
    models::Login,
    utils::{
        get_web_player_app_id, get_web_player_app_secret, read_app_credentials_from_env,
        write_app_credentials_to_env,
    },
};

/// Constants for the Qobuz API
pub mod constants {
    /// Base URL for the Qobuz API
    ///
    /// This is the main endpoint for all Qobuz API requests. The API version is included in the URL.
    /// All API calls should be made relative to this base URL.
    pub const API_BASE_URL: &str = "https://www.qobuz.com/api.json/0.2";
    /// Base URL for the Qobuz Web Player
    ///
    /// This URL is used to extract application credentials from the Qobuz web player.
    /// The library fetches app ID and app secret from the web player's JavaScript bundle.
    pub const WEB_PLAYER_BASE_URL: &str = "https://play.qobuz.com";
}

/// The service disclosing the various endpoints of the Qobuz REST API.
///
/// The service can be initialized using your own 'app_id' and 'app_secret',
/// or by letting the service attempt to fetch these 2 values from the Qobuz Web Player.
///
/// # Examples
///
/// Basic initialization with automatic credential fetching:
///
/// ```no_run
/// use qobuz_api_rust::QobuzApiService;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let service = QobuzApiService::new().await?;
///     Ok(())
/// }
/// ```
///
/// Initialization with custom credentials:
///
/// ```no_run
/// use qobuz_api_rust::QobuzApiService;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     let service = QobuzApiService::with_credentials(
///         Some("your_app_id".to_string()),
///         Some("your_app_secret".to_string())
///     ).await?;
///     Ok(())
/// }
/// ```
pub struct QobuzApiService {
    /// The application ID for the Qobuz API
    ///
    /// This is a unique identifier for your application registered with Qobuz.
    /// It's used in API requests to identify the source of the request.
    pub app_id: String,
    /// The application secret for the Qobuz API
    ///
    /// This is a secret key associated with your application ID.
    /// It's used to sign requests and authenticate with the API.
    pub app_secret: String,
    /// The user authentication token, if authenticated
    ///
    /// This token is obtained after successful user authentication and is used
    /// for API requests that require user context.
    pub user_auth_token: Option<String>,
    /// HTTP client used for making API requests
    ///
    /// This client is configured with appropriate headers and user agent
    /// for making requests to the Qobuz API.
    pub(crate) client: Client,
}

impl QobuzApiService {
    /// Initializes a new instance of the QobuzApiService using cached credentials from .env file
    /// or by dynamically retrieving them from the Qobuz Web Player if not available.
    ///
    /// This method attempts to initialize the service in the following order:
    /// 1. Try to use cached credentials from the .env file
    /// 2. If cached credentials fail, fetch new ones from the web player
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to fetch credentials from the web player
    /// - Failed to create an HTTP client
    /// - Both cached and fetched credentials are invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qobuz_api_rust::QobuzApiService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let service = QobuzApiService::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self, QobuzApiError> {
        // First, try to read credentials from .env file
        if let Ok((Some(cached_app_id), Some(cached_app_secret))) = read_app_credentials_from_env()
        {
            if !cached_app_id.is_empty() && !cached_app_secret.is_empty() {
                println!("Using cached credentials from .env file");

                // Try to initialize with cached credentials
                match Self::with_credentials(
                    Some(cached_app_id.clone()),
                    Some(cached_app_secret.clone()),
                )
                .await
                {
                    Ok(service) => {
                        return Ok(service);
                    }
                    Err(e) => {
                        println!(
                            "Cached credentials failed to initialize ({}), fetching new ones...",
                            e
                        );
                    }
                }
            }
        } else {
            println!("No cached credentials found, fetching new ones...");
        }

        // Fetch fresh credentials from web player
        let app_id = get_web_player_app_id()
            .await
            .map_err(|e| QobuzApiInitializationError {
                message: format!("Failed to fetch app ID from web player: {}", e),
            })?;

        let app_secret =
            get_web_player_app_secret()
                .await
                .map_err(|e| QobuzApiInitializationError {
                    message: format!("Failed to fetch app secret from web player: {}", e),
                })?;

        // Store the fetched credentials in .env file for future use
        if let Err(e) = write_app_credentials_to_env(&app_id, &app_secret) {
            eprintln!("Warning: Failed to write credentials to .env file: {}", e);
        } else {
            println!("Successfully stored new credentials in .env file");
        }

        Self::with_credentials(Some(app_id), Some(app_secret)).await
    }

    /// Initializes a new instance of the QobuzApiService with custom app_id and app_secret.
    ///
    /// This method allows you to provide your own application credentials instead of
    /// automatically fetching them from the web player.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Optional application ID. If None, initialization will fail.
    /// * `app_secret` - Optional application secret. If None, initialization will fail.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Either app_id or app_secret is None
    /// - Failed to create an HTTP client with the provided credentials
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qobuz_api_rust::QobuzApiService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let service = QobuzApiService::with_credentials(
    ///         Some("your_app_id".to_string()),
    ///         Some("your_app_secret".to_string())
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn with_credentials(
        app_id: Option<String>,
        app_secret: Option<String>,
    ) -> Result<Self, QobuzApiError> {
        let has_credentials = app_id.is_some() && app_secret.is_some();

        if !has_credentials {
            return Err(CredentialsError {
                message: "App ID and App Secret must be provided".to_string(),
            });
        }

        let app_id = match app_id {
            Some(id) if !id.is_empty() => id,
            _ => {
                return Err(CredentialsError {
                    message: "App ID cannot be empty".to_string(),
                });
            }
        };

        let app_secret = match app_secret {
            Some(secret) if !secret.is_empty() => secret,
            _ => {
                return Err(CredentialsError {
                    message: "App Secret cannot be empty".to_string(),
                });
            }
        };

        let client = Client::builder()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/110.0",
            )
            .default_headers(
                once((
                    HeaderName::from_static("x-app-id"),
                    HeaderValue::from_str(&app_id).map_err(|e| QobuzApiInitializationError {
                        message: format!("Failed to create header value for app ID: {}", e),
                    })?,
                ))
                .collect(),
            )
            .build()
            .map_err(HttpError)?;

        let service = QobuzApiService {
            app_id,
            app_secret,
            user_auth_token: None,
            client,
        };

        Ok(service)
    }

    /// Sets the user authentication token for the service
    ///
    /// This method is used to set the user authentication token after successful
    /// user authentication. The token will be used for subsequent API requests
    /// that require user context.
    ///
    /// # Arguments
    ///
    /// * `token` - The user authentication token to set
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qobuz_api_rust::QobuzApiService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut service = QobuzApiService::new().await?;
    ///     service.set_user_auth_token("your_auth_token".to_string());
    ///     Ok(())
    /// }
    /// ```
    pub fn set_user_auth_token(&mut self, token: String) {
        self.user_auth_token = Some(token);
    }

    /// Internal helper to perform authentication with given credentials
    ///
    /// This method will try different authentication methods based on the provided parameters:
    /// 1. If both user_id and user_auth_token are provided, use token-based authentication
    /// 2. If email and password are provided, use identifier/password authentication
    /// 3. If username and password are provided, use identifier/password authentication
    ///
    /// # Arguments
    ///
    /// * `user_id` - Optional user ID for token-based authentication
    /// * `user_auth_token` - Optional user authentication token
    /// * `email` - Optional email for email/password authentication
    /// * `password` - Optional password (MD5 hashed) for email/username authentication
    /// * `username` - Optional username for username/password authentication
    ///
    /// # Returns
    ///
    /// * `Ok(Login)` - Login response containing user and auth token
    /// * `Err(QobuzApiError)` - If authentication fails or no valid credentials are provided
    ///
    /// # Errors
    ///
    /// Returns an error if no valid combination of credentials is provided or if authentication fails.
    pub(super) async fn authenticate_with_creds(
        &mut self,
        user_id: Option<&str>,
        user_auth_token: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
        username: Option<&str>,
    ) -> Result<Login, QobuzApiError> {
        // Check if both user_id and user_auth_token are provided
        if let (Some(uid), Some(token)) = (user_id, user_auth_token)
            && !uid.is_empty()
            && !token.is_empty()
        {
            println!("Using token-based authentication");
            return self.login_with_token(uid, token).await;
        }

        // Check if both email and password are provided
        if let (Some(em), Some(pwd)) = (email, password)
            && !em.is_empty()
            && !pwd.is_empty()
        {
            println!("Using email/password authentication");
            return self.login(em, pwd).await;
        }

        // Check if both username and password are provided
        if let (Some(un), Some(pwd)) = (username, password)
            && !un.is_empty()
            && !pwd.is_empty()
        {
            println!("Using username/password authentication");
            return self.login(un, pwd).await;
        }

        // If no valid combination of credentials is provided, return an error
        Err(AuthenticationError {
            message: "No valid authentication credentials provided. Please provide either: (user_id and user_auth_token) or (email and password) or (username and password)".to_string(),
        })
    }

    /// Attempts to authenticate using environment variables.
    ///
    /// Checks for QOBUZ_USER_ID and QOBUZ_USER_AUTH_TOKEN first,
    /// then falls back to QOBUZ_EMAIL and QOBUZ_PASSWORD,
    /// then to QOBUZ_USERNAME and QOBUZ_PASSWORD.
    /// Both email and username are treated as identifiers for authentication.
    ///
    /// # Returns
    ///
    /// * `Ok(Login)` - If authentication was successful
    /// * `Err(QobuzApiError)` - If authentication failed or no valid credentials were found
    ///
    /// # Environment Variables
    ///
    /// This method looks for the following environment variables:
    /// - `QOBUZ_USER_ID` and `QOBUZ_USER_AUTH_TOKEN` for token-based authentication
    /// - `QOBUZ_EMAIL` and `QOBUZ_PASSWORD` for email/password authentication
    /// - `QOBUZ_USERNAME` and `QOBUZ_PASSWORD` for username/password authentication
    ///
    /// # Errors
    ///
    /// Returns an error if no valid credentials are found in environment variables
    /// or if authentication fails with the provided credentials.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qobuz_api_rust::QobuzApiService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut service = QobuzApiService::new().await?;
    ///     let login_result = service.authenticate_with_env().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn authenticate_with_env(&mut self) -> Result<Login, QobuzApiError> {
        // Read environment variables
        let user_id = var("QOBUZ_USER_ID").ok();
        let user_auth_token = var("QOBUZ_USER_AUTH_TOKEN").ok();
        let email = var("QOBUZ_EMAIL").ok();
        let password = var("QOBUZ_PASSWORD").ok();
        let username = var("QOBUZ_USERNAME").ok();

        // Use the shared authentication helper function
        self.authenticate_with_creds(
            user_id.as_deref(),
            user_auth_token.as_deref(),
            email.as_deref(),
            password.as_deref(),
            username.as_deref(),
        )
        .await
    }

    /// Refreshes the app credentials by fetching new ones from the web player and updating the .env file
    ///
    /// This method creates a new instance of the service with fresh credentials
    /// obtained from the Qobuz web player, while preserving the current user authentication token.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to fetch new credentials from the web player
    /// - Failed to create a new service instance with the fetched credentials
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use qobuz_api_rust::QobuzApiService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let service = QobuzApiService::new().await?;
    ///     let updated_service = service.refresh_app_credentials().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn refresh_app_credentials(&self) -> Result<Self, QobuzApiError> {
        println!("Fetching new app credentials from web player...");

        // Fetch fresh credentials from web player
        let app_id = get_web_player_app_id()
            .await
            .map_err(|e| QobuzApiInitializationError {
                message: format!("Failed to fetch app ID from web player: {}", e),
            })?;

        let app_secret =
            get_web_player_app_secret()
                .await
                .map_err(|e| QobuzApiInitializationError {
                    message: format!("Failed to fetch app secret from web player: {}", e),
                })?;

        // Store the new credentials in .env file
        if let Err(e) = write_app_credentials_to_env(&app_id, &app_secret) {
            eprintln!("Warning: Failed to update credentials in .env file: {}", e);
        } else {
            println!("Successfully updated credentials in .env file");
        }

        // Create a new service instance with the updated credentials and preserve the user auth token
        let mut new_service = Self::with_credentials(Some(app_id), Some(app_secret)).await?;
        new_service.user_auth_token = self.user_auth_token.clone();

        Ok(new_service)
    }
}

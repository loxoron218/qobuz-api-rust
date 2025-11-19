use {
    serde::{Deserialize, Deserializer, Serialize, de::Error},
    serde_json::Value::{self, Null},
};

use crate::models::User;

/// Custom deserializer for the 'code' field that handles both string and numeric values
///
/// This function allows deserializing the 'code' field from either a string or numeric value,
/// converting numeric values to strings. This is useful when the API may return codes
/// in different formats.
///
/// # Arguments
///
/// * `deserializer` - The deserializer to use for reading the value
///
/// # Returns
///
/// A `Result` containing either an `Option<String>` with the code value or a deserialization error
pub fn deserialize_code<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(Some(s)),

        Value::Number(n) => Ok(Some(n.to_string())),

        Null => Ok(None),

        _ => Err(Error::custom("Invalid code format")),
    }
}

/// A general Qobuz API status response model
///
/// This struct represents the standard response format for Qobuz API calls that return
/// status information, including an optional code, message, and status string.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::QobuzApiStatusResponse;
///
/// let response = QobuzApiStatusResponse::new(
///     Some("success".to_string()),
///     Some("Operation completed successfully".to_string()),
///     Some("ok".to_string())
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct QobuzApiStatusResponse {
    /// The status code of the API response, which can be a string or numeric value
    #[serde(rename = "code", deserialize_with = "deserialize_code")]
    pub code: Option<String>,

    /// A human-readable message describing the status of the response
    #[serde(rename = "message")]
    pub message: Option<String>,

    /// The status string indicating the result of the API call
    #[serde(rename = "status")]
    pub status: Option<String>,
}

impl QobuzApiStatusResponse {
    /// Creates a new instance of `QobuzApiStatusResponse`
    ///
    /// # Arguments
    ///
    /// * `code` - Optional status code for the response
    /// * `message` - Optional human-readable message
    /// * `status` - Optional status string
    ///
    /// # Returns
    ///
    /// A new instance of `QobuzApiStatusResponse` with the provided values
    pub fn new(code: Option<String>, message: Option<String>, status: Option<String>) -> Self {
        QobuzApiStatusResponse {
            code,
            message,
            status,
        }
    }
}

/// Login response model containing user and authentication token
///
/// This struct represents the response from a login operation, containing the
/// user information and authentication token.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{Login, User};
///
/// let login_response = Login {
///     user: Some(User::default()),
///     auth_token: Some("auth_token_123".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Login {
    /// User information returned with the login response
    #[serde(rename = "user")]
    pub user: Option<User>,

    /// Authentication token for the logged-in user
    #[serde(rename = "user_auth_token")]
    pub auth_token: Option<String>,
}

use serde::{Deserialize, Serialize};

use crate::models::{Credential, LastUpdate, StoreFeatures, Subscription};

/// User model representing a Qobuz user
///
/// This struct contains comprehensive information about a Qobuz user including their
/// identification, personal information, account details, and subscription information.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::User;
///
/// let user = User {
///     id: Some(123456789),
///     display_name: Some("Example User".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    /// Unique identifier for the user
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// Public identifier for the user
    #[serde(rename = "publicId")]
    pub public_id: Option<String>,

    /// Display name for the user
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,

    /// First name of the user
    #[serde(rename = "firstname")]
    pub firstname: Option<String>,

    /// Last name of the user
    #[serde(rename = "lastname")]
    pub lastname: Option<String>,

    /// Email address of the user
    #[serde(rename = "email")]
    pub email: Option<String>,

    /// Login username of the user
    #[serde(rename = "login")]
    pub login: Option<String>,

    /// Age of the user
    #[serde(rename = "age")]
    pub age: Option<i64>,

    /// Genre preference of the user
    #[serde(rename = "genre")]
    pub genre: Option<String>,

    /// Country where the user is located
    #[serde(rename = "country")]
    pub country: Option<String>,

    /// Country code for the user
    #[serde(rename = "country_code")]
    pub country_code: Option<String>,

    /// Language code for the user
    #[serde(rename = "language_code")]
    pub language_code: Option<String>,

    /// Zone information for the user
    #[serde(rename = "zone")]
    pub zone: Option<String>,

    /// Store associated with the user
    #[serde(rename = "store")]
    pub store: Option<String>,

    /// URL to the user's avatar image
    #[serde(rename = "avatar")]
    pub avatar: Option<String>, // Using String instead of Uri

    /// Date when the user account was created
    #[serde(rename = "creation_date")]
    pub creation_date: Option<String>,

    /// Credential information for the user
    #[serde(rename = "credential")]
    pub credential: Option<Credential>,

    /// Last update information for the user
    #[serde(rename = "last_update")]
    pub last_update: Option<LastUpdate>,

    /// Store features available to the user
    #[serde(rename = "store_features")]
    pub store_features: Option<StoreFeatures>,

    /// Subscription information for the user
    #[serde(rename = "subscription")]
    pub subscription: Option<Subscription>,
}

use serde::{Deserialize, Serialize};

/// Last update model containing timestamps for various user data
///
/// This struct represents the last update times for different types of user data
/// such as favorites, playlists, and other collections.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::LastUpdate;
///
/// let last_update = LastUpdate {
///     favorites: Some("2023-01-01T00:00:00Z".to_string()),
///     playlists: Some("2023-01-02T00:00:00Z".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LastUpdate {
    /// Last update time for favorites
    #[serde(rename = "favorites")]
    pub favorites: Option<String>,

    /// Last update time for playlists
    #[serde(rename = "playlists")]
    pub playlists: Option<String>,

    /// Last update time for followings
    #[serde(rename = "followings")]
    pub followings: Option<String>,

    /// Last update time for subscriptions
    #[serde(rename = "subscriptions")]
    pub subscriptions: Option<String>,

    /// Last update time for purchases
    #[serde(rename = "purchases")]
    pub purchases: Option<String>,

    /// Last update time for playback history
    #[serde(rename = "playback_history")]
    pub playback_history: Option<String>,

    /// Last update time for the user's library
    #[serde(rename = "library")]
    pub library: Option<String>,

    /// Last update time for recommendations
    #[serde(rename = "recommendations")]
    pub recommendations: Option<String>,

    /// Last update time for discover content
    #[serde(rename = "discover")]
    pub discover: Option<String>,

    /// Last update time for personal radio
    #[serde(rename = "personal_radio")]
    pub personal_radio: Option<String>,

    /// Last update time for instant mixes
    #[serde(rename = "instant_mixes")]
    pub instant_mixes: Option<String>,

    /// Last update time for smart playlists
    #[serde(rename = "smart_playlists")]
    pub smart_playlists: Option<String>,

    /// Last update time for daily mixes
    #[serde(rename = "daily_mixes")]
    pub daily_mixes: Option<String>,

    /// Last update time for weekly mixes
    #[serde(rename = "weekly_mixes")]
    pub weekly_mixes: Option<String>,

    /// Last update time for monthly mixes
    #[serde(rename = "monthly_mixes")]
    pub monthly_mixes: Option<String>,

    /// Last update time for yearly mixes
    #[serde(rename = "yearly_mixes")]
    pub yearly_mixes: Option<String>,
}

/// Store features model containing information about available features
///
/// This struct represents the features available in the Qobuz store for a user
/// including free tier, trial, subscription, and various content types.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::StoreFeatures;
///
/// let store_features = StoreFeatures {
///     has_free_tier: Some(true),
///     has_subscription: Some(true),
///     has_download: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct StoreFeatures {
    /// Whether the store has a free tier
    #[serde(rename = "has_free_tier")]
    pub has_free_tier: Option<bool>,

    /// Whether the store has a trial option
    #[serde(rename = "has_trial")]
    pub has_trial: Option<bool>,

    /// Whether the store has a subscription option
    #[serde(rename = "has_subscription")]
    pub has_subscription: Option<bool>,

    /// Whether the store offers high-resolution content
    #[serde(rename = "has_hires")]
    pub has_hires: Option<bool>,

    /// Whether the store allows downloading
    #[serde(rename = "has_download")]
    pub has_download: Option<bool>,

    /// Whether the store allows streaming
    #[serde(rename = "has_streaming")]
    pub has_streaming: Option<bool>,

    /// Whether the store has radio functionality
    #[serde(rename = "has_radio")]
    pub has_radio: Option<bool>,

    /// Whether the store has playlist functionality
    #[serde(rename = "has_playlist")]
    pub has_playlist: Option<bool>,

    /// Whether the store has favorites functionality
    #[serde(rename = "has_favorites")]
    pub has_favorites: Option<bool>,

    /// Whether the store has a library functionality
    #[serde(rename = "has_library")]
    pub has_library: Option<bool>,

    /// Whether the store has discovery features
    #[serde(rename = "has_discover")]
    pub has_discover: Option<bool>,

    /// Whether the store has recommendation features
    #[serde(rename = "has_recommendations")]
    pub has_recommendations: Option<bool>,

    /// Whether the store has personal radio features
    #[serde(rename = "has_personal_radio")]
    pub has_personal_radio: Option<bool>,

    /// Whether the store has instant mixes
    #[serde(rename = "has_instant_mixes")]
    pub has_instant_mixes: Option<bool>,

    /// Whether the store has smart playlists
    #[serde(rename = "has_smart_playlists")]
    pub has_smart_playlists: Option<bool>,

    /// Whether the store has daily mixes
    #[serde(rename = "has_daily_mixes")]
    pub has_daily_mixes: Option<bool>,

    /// Whether the store has weekly mixes
    #[serde(rename = "has_weekly_mixes")]
    pub has_weekly_mixes: Option<bool>,

    /// Whether the store has monthly mixes
    #[serde(rename = "has_monthly_mixes")]
    pub has_monthly_mixes: Option<bool>,

    /// Whether the store has yearly mixes
    #[serde(rename = "has_yearly_mixes")]
    pub has_yearly_mixes: Option<bool>,
}

/// Subscription model containing user subscription information
///
/// This struct represents a user's subscription details including plan information,
/// status, dates, and payment information.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Subscription;
///
/// let subscription = Subscription {
///     id: Some("sub123".to_string()),
///     status: Some("active".to_string()),
///     is_active: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Subscription {
    /// Unique identifier for the subscription
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// User ID associated with the subscription
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,

    /// Offer ID for the subscription plan
    #[serde(rename = "offer_id")]
    pub offer_id: Option<String>,

    /// Name of the subscription offer
    #[serde(rename = "offer_name")]
    pub offer_name: Option<String>,

    /// Country where the offer is available
    #[serde(rename = "offer_country")]
    pub offer_country: Option<String>,

    /// Currency used for the subscription
    #[serde(rename = "offer_currency")]
    pub offer_currency: Option<String>,

    /// Price of the subscription offer
    #[serde(rename = "offer_price")]
    pub offer_price: Option<f64>,

    /// Number of trial days included in the offer
    #[serde(rename = "offer_trial_days")]
    pub offer_trial_days: Option<i32>,

    /// Type of the subscription offer
    #[serde(rename = "offer_type")]
    pub offer_type: Option<String>,

    /// Family of the subscription offer
    #[serde(rename = "offer_family")]
    pub offer_family: Option<String>,

    /// List of features included in the offer
    #[serde(rename = "offer_features")]
    pub offer_features: Option<Vec<String>>,

    /// Current status of the subscription
    #[serde(rename = "status")]
    pub status: Option<String>,

    /// Status code for the subscription
    #[serde(rename = "status_code")]
    pub status_code: Option<String>,

    /// Start date of the subscription
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,

    /// End date of the subscription
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,

    /// Date when the subscription will renew
    #[serde(rename = "renew_date")]
    pub renew_date: Option<String>,

    /// Date when the subscription was canceled
    #[serde(rename = "cancel_date")]
    pub cancel_date: Option<String>,

    /// Reason for cancellation
    #[serde(rename = "cancel_reason")]
    pub cancel_reason: Option<String>,

    /// Whether the subscription is currently active
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,

    /// Whether the subscription is currently in trial
    #[serde(rename = "is_trial")]
    pub is_trial: Option<bool>,

    /// Whether the subscription has been canceled
    #[serde(rename = "is_canceled")]
    pub is_canceled: Option<bool>,

    /// Whether the subscription has expired
    #[serde(rename = "is_expired")]
    pub is_expired: Option<bool>,

    /// Whether the subscription can be renewed
    #[serde(rename = "is_renewable")]
    pub is_renewable: Option<bool>,

    /// Whether the subscription has auto-renewal enabled
    #[serde(rename = "is_auto_renew")]
    pub is_auto_renew: Option<bool>,

    /// Date of the next payment
    #[serde(rename = "next_payment_date")]
    pub next_payment_date: Option<String>,

    /// Date of the last payment
    #[serde(rename = "last_payment_date")]
    pub last_payment_date: Option<String>,

    /// Amount of the next payment
    #[serde(rename = "next_payment_amount")]
    pub next_payment_amount: Option<f64>,

    /// Payment method used for the subscription
    #[serde(rename = "payment_method")]
    pub payment_method: Option<String>,

    /// Status of the payment
    #[serde(rename = "payment_status")]
    pub payment_status: Option<String>,

    /// Number of failed payment attempts
    #[serde(rename = "payment_failed_count")]
    pub payment_failed_count: Option<i32>,

    /// Reason for payment failure
    #[serde(rename = "payment_failed_reason")]
    pub payment_failed_reason: Option<String>,

    /// Date of the payment failure
    #[serde(rename = "payment_failed_date")]
    pub payment_failed_date: Option<String>,

    /// Date for the next payment retry
    #[serde(rename = "payment_retry_date")]
    pub payment_retry_date: Option<String>,

    /// Number of payment retry attempts made
    #[serde(rename = "payment_retry_count")]
    pub payment_retry_count: Option<i32>,

    /// Maximum number of payment retry attempts allowed
    #[serde(rename = "payment_retry_max")]
    pub payment_retry_max: Option<i32>,

    /// Interval between payment retry attempts
    #[serde(rename = "payment_retry_interval")]
    pub payment_retry_interval: Option<i32>,

    /// Multiplier for payment retry intervals
    #[serde(rename = "payment_retry_multiplier")]
    pub payment_retry_multiplier: Option<f64>,

    /// Backoff strategy for payment retries
    #[serde(rename = "payment_retry_backoff")]
    pub payment_retry_backoff: Option<String>,

    /// Strategy for payment retries
    #[serde(rename = "payment_retry_strategy")]
    pub payment_retry_strategy: Option<String>,

    /// Whether payment retry is enabled
    #[serde(rename = "payment_retry_enabled")]
    pub payment_retry_enabled: Option<bool>,

    /// Whether payment retry is currently active
    #[serde(rename = "payment_retry_active")]
    pub payment_retry_active: Option<bool>,

    /// Whether payment retry is scheduled
    #[serde(rename = "payment_retry_scheduled")]
    pub payment_retry_scheduled: Option<bool>,

    /// Date when payment retry is scheduled
    #[serde(rename = "payment_retry_scheduled_date")]
    pub payment_retry_scheduled_date: Option<String>,

    /// Number of scheduled payment retry attempts
    #[serde(rename = "payment_retry_scheduled_count")]
    pub payment_retry_scheduled_count: Option<i32>,

    /// Reason for scheduled payment retry
    #[serde(rename = "payment_retry_scheduled_reason")]
    pub payment_retry_scheduled_reason: Option<String>,
}

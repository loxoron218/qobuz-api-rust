use serde::{Deserialize, Serialize};

/// Credential model containing user credential information
///
/// This struct represents comprehensive user credential information including
/// personal details, account settings, and feature availability.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Credential;
///
/// let credential = Credential {
///     user_id: Some(123456789),
///     user_display_name: Some("Example User".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Credential {
    /// User ID associated with the credentials
    #[serde(rename = "user_id")]
    pub user_id: Option<i64>,

    /// Display name of the user
    #[serde(rename = "user_display_name")]
    pub user_display_name: Option<String>,

    /// Login username of the user
    #[serde(rename = "user_login")]
    pub user_login: Option<String>,

    /// Email address of the user
    #[serde(rename = "user_mail")]
    pub user_mail: Option<String>,

    /// First name of the user
    #[serde(rename = "user_firstname")]
    pub user_firstname: Option<String>,

    /// Last name of the user
    #[serde(rename = "user_lastname")]
    pub user_lastname: Option<String>,

    /// Gender of the user
    #[serde(rename = "user_gender")]
    pub user_gender: Option<String>,

    /// Birthdate of the user
    #[serde(rename = "user_birthdate")]
    pub user_birthdate: Option<String>,

    /// Country where the user is located
    #[serde(rename = "user_country")]
    pub user_country: Option<String>,

    /// Timezone of the user
    #[serde(rename = "user_timezone")]
    pub user_timezone: Option<String>,

    /// Language of the user
    #[serde(rename = "user_language")]
    pub user_language: Option<String>,

    /// URL to the user's avatar
    #[serde(rename = "user_avatar")]
    pub user_avatar: Option<String>,

    /// Whether the user profile is partial
    #[serde(rename = "user_partial")]
    pub user_partial: Option<bool>,

    /// Age of the user
    #[serde(rename = "user_age")]
    pub user_age: Option<i64>,

    /// Date when the user account was created
    #[serde(rename = "user_created_at")]
    pub user_created_at: Option<String>,

    /// Whether the user is anonymous
    #[serde(rename = "user_is_anonymous")]
    pub user_is_anonymous: Option<bool>,

    /// Whether the user is a subscriber
    #[serde(rename = "user_is_subscriber")]
    pub user_is_subscriber: Option<bool>,

    /// Whether the user is on a trial
    #[serde(rename = "user_is_trialist")]
    pub user_is_trialist: Option<bool>,

    /// Whether the user has a free account
    #[serde(rename = "user_is_free")]
    pub user_is_free: Option<bool>,

    /// Type of subscription the user has
    #[serde(rename = "user_subscription_type")]
    pub user_subscription_type: Option<String>,

    /// Store associated with the user
    #[serde(rename = "user_store")]
    pub user_store: Option<String>,

    /// Zone associated with the user
    #[serde(rename = "user_zone")]
    pub user_zone: Option<String>,

    /// Date when the user's trial expires
    #[serde(rename = "user_trial_expire_at")]
    pub user_trial_expire_at: Option<String>,

    /// Number of days left in the user's trial
    #[serde(rename = "user_trial_days_left")]
    pub user_trial_days_left: Option<i64>,

    /// Whether the user has used a trial
    #[serde(rename = "user_trial_used")]
    pub user_trial_used: Option<bool>,

    /// Whether the user has a promotion
    #[serde(rename = "user_has_promo")]
    pub user_has_promo: Option<bool>,

    /// Number of days left in the user's promotion
    #[serde(rename = "user_promo_days_left")]
    pub user_promo_days_left: Option<i64>,

    /// Whether the user has an inbox
    #[serde(rename = "user_has_inbox")]
    pub user_has_inbox: Option<bool>,

    /// Number of unread messages in the user's inbox
    #[serde(rename = "user_inbox_unread_count")]
    pub user_inbox_unread_count: Option<i32>,

    /// Whether the user has newsletter subscriptions
    #[serde(rename = "user_has_newsletter")]
    pub user_has_newsletter: Option<bool>,

    /// Whether the user is subscribed to Qobuz newsletter
    #[serde(rename = "user_has_newsletter_qobuz")]
    pub user_has_newsletter_qobuz: Option<bool>,

    /// Whether the user is subscribed to label newsletter
    #[serde(rename = "user_has_newsletter_label")]
    pub user_has_newsletter_label: Option<bool>,

    /// Whether the user is subscribed to partner newsletter
    #[serde(rename = "user_has_newsletter_partner")]
    pub user_has_newsletter_partner: Option<bool>,

    /// Whether the user is subscribed to third-party newsletter
    #[serde(rename = "user_has_newsletter_third_party")]
    pub user_has_newsletter_third_party: Option<bool>,

    /// Whether the user can stream high-resolution audio
    #[serde(rename = "user_can_stream_hires")]
    pub user_can_stream_hires: Option<bool>,

    /// Whether the user can download high-resolution audio
    #[serde(rename = "user_can_download_hires")]
    pub user_can_download_hires: Option<bool>,

    /// Whether the user can skip tracks
    #[serde(rename = "user_can_skip_tracks")]
    pub user_can_skip_tracks: Option<bool>,

    /// Whether the user can repeat tracks
    #[serde(rename = "user_can_repeat_tracks")]
    pub user_can_repeat_tracks: Option<bool>,

    /// Whether the user can create playlists
    #[serde(rename = "user_can_create_playlist")]
    pub user_can_create_playlist: Option<bool>,

    /// Whether the user can create public playlists
    #[serde(rename = "user_can_create_public_playlist")]
    pub user_can_create_public_playlist: Option<bool>,

    /// Whether the user can create radio stations
    #[serde(rename = "user_can_create_radio")]
    pub user_can_create_radio: Option<bool>,

    /// Whether the user can share content
    #[serde(rename = "user_can_share")]
    pub user_can_share: Option<bool>,

    /// Whether the user can download content
    #[serde(rename = "user_can_download")]
    pub user_can_download: Option<bool>,

    /// Whether the user has access to instant mixes
    #[serde(rename = "user_has_instant_mix")]
    pub user_has_instant_mix: Option<bool>,

    /// Whether the user has access to radio
    #[serde(rename = "user_has_radio")]
    pub user_has_radio: Option<bool>,

    /// Whether the user has access to discovery features
    #[serde(rename = "user_has_discovery")]
    pub user_has_discovery: Option<bool>,

    /// Whether the user has access to personalization features
    #[serde(rename = "user_has_personalization")]
    pub user_has_personalization: Option<bool>,

    /// Whether the user has access to replay gain
    #[serde(rename = "user_has_replaygain")]
    pub user_has_replaygain: Option<bool>,

    /// Whether the user has access to equalizer
    #[serde(rename = "user_has_equalizer")]
    pub user_has_equalizer: Option<bool>,

    /// Whether the user has access to crossfade
    #[serde(rename = "user_has_crossfade")]
    pub user_has_crossfade: Option<bool>,

    /// Whether the user has access to gapless playback
    #[serde(rename = "user_has_gapless_playback")]
    pub user_has_gapless_playback: Option<bool>,

    /// Whether the user has access to offline mode
    #[serde(rename = "user_has_offline_mode")]
    pub user_has_offline_mode: Option<bool>,

    /// Whether the user has access to lyrics
    #[serde(rename = "user_has_lyrics")]
    pub user_has_lyrics: Option<bool>,

    /// Whether the user has access to captions
    #[serde(rename = "user_has_captions")]
    pub user_has_captions: Option<bool>,

    /// Whether the user has access to audio quality settings
    #[serde(rename = "user_has_audio_quality_settings")]
    pub user_has_audio_quality_settings: Option<bool>,

    /// Whether the user has access to UI settings
    #[serde(rename = "user_has_ui_settings")]
    pub user_has_ui_settings: Option<bool>,

    /// Whether the user has access to account settings
    #[serde(rename = "user_has_account_settings")]
    pub user_has_account_settings: Option<bool>,

    /// Whether the user has access to privacy settings
    #[serde(rename = "user_has_privacy_settings")]
    pub user_has_privacy_settings: Option<bool>,

    /// Whether the user has access to subscription settings
    #[serde(rename = "user_has_subscription_settings")]
    pub user_has_subscription_settings: Option<bool>,

    /// Whether the user has payment methods set up
    #[serde(rename = "user_has_payment_methods")]
    pub user_has_payment_methods: Option<bool>,

    /// Whether the user has addresses saved
    #[serde(rename = "user_has_addresses")]
    pub user_has_addresses: Option<bool>,

    /// Whether the user has playback history
    #[serde(rename = "user_has_playback_history")]
    pub user_has_playback_history: Option<bool>,

    /// Whether the user has a library
    #[serde(rename = "user_has_library")]
    pub user_has_library: Option<bool>,

    /// Whether the user has favorites
    #[serde(rename = "user_has_favorites")]
    pub user_has_favorites: Option<bool>,

    /// Whether the user has playlists
    #[serde(rename = "user_has_playlists")]
    pub user_has_playlists: Option<bool>,

    /// Whether the user has followings
    #[serde(rename = "user_has_followings")]
    pub user_has_followings: Option<bool>,

    /// Whether the user has subscriptions
    #[serde(rename = "user_has_subscriptions")]
    pub user_has_subscriptions: Option<bool>,

    /// Whether the user has purchases
    #[serde(rename = "user_has_purchases")]
    pub user_has_purchases: Option<bool>,

    /// Whether the user has playback statistics
    #[serde(rename = "user_has_playback_statistics")]
    pub user_has_playback_statistics: Option<bool>,

    /// Whether the user has recommendations
    #[serde(rename = "user_has_recommendations")]
    pub user_has_recommendations: Option<bool>,

    /// Whether the user has access to discover features
    #[serde(rename = "user_has_discover")]
    pub user_has_discover: Option<bool>,

    /// Whether the user has access to personal radio
    #[serde(rename = "user_has_personal_radio")]
    pub user_has_personal_radio: Option<bool>,

    /// Whether the user has access to instant mixes
    #[serde(rename = "user_has_instant_mixes")]
    pub user_has_instant_mixes: Option<bool>,

    /// Whether the user has access to smart playlists
    #[serde(rename = "user_has_smart_playlists")]
    pub user_has_smart_playlists: Option<bool>,

    /// Whether the user has access to daily mixes
    #[serde(rename = "user_has_daily_mixes")]
    pub user_has_daily_mixes: Option<bool>,

    /// Whether the user has access to weekly mixes
    #[serde(rename = "user_has_weekly_mixes")]
    pub user_has_weekly_mixes: Option<bool>,

    /// Whether the user has access to monthly mixes
    #[serde(rename = "user_has_monthly_mixes")]
    pub user_has_monthly_mixes: Option<bool>,

    /// Whether the user has access to yearly mixes
    #[serde(rename = "user_has_yearly_mixes")]
    pub user_has_yearly_mixes: Option<bool>,
}

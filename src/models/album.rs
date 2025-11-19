use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

use crate::models::{
    AlbumsSameArtist, Area, Article, Artist, Award, Focus, Genre, Goody, Image, ItemSearchResult,
    Label, Period, Track,
};

/// Album model representing an album on the Qobuz platform
///
/// This struct contains comprehensive information about an album including its
/// identification, title, artists, label, genre, image, and various metadata.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Album;
///
/// let album = Album {
///     id: Some("123456".to_string()),
///     title: Some("Example Album".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Album {
    /// Unique identifier for the album
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Title of the album
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Subtitle of the album (if any)
    #[serde(rename = "subtitle")]
    pub subtitle: Option<String>,

    /// Version information for the album (e.g., "Deluxe Edition")
    #[serde(rename = "version")]
    pub version: Option<String>,

    /// Universal Product Code for the album
    #[serde(rename = "upc")]
    pub upc: Option<String>,

    /// URL to the album on Qobuz
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// URL to the product page for the album
    #[serde(rename = "product_url")]
    pub product_url: Option<String>,

    /// Relative URL to the album
    #[serde(rename = "relative_url")]
    pub relative_url: Option<String>,

    /// Main artist associated with the album (boxed to handle recursive structures)
    #[serde(rename = "artist")]
    pub artist: Option<Box<Artist>>,

    /// List of all artists associated with the album (boxed to handle recursive structures)
    #[serde(rename = "artists")]
    pub artists: Option<Vec<Box<Artist>>>,

    /// Composer of the album (boxed to handle recursive structures)
    #[serde(rename = "composer")]
    pub composer: Option<Box<Artist>>,

    /// Label information for the album
    #[serde(rename = "label")]
    pub label: Option<Label>,

    /// Genre information for the album
    #[serde(rename = "genre")]
    pub genre: Option<Genre>,

    /// List of genre names associated with the album
    #[serde(rename = "genres_list")]
    pub genres_list: Option<Vec<String>>,

    /// Image information for the album artwork
    #[serde(rename = "image")]
    pub image: Option<Image>,

    /// Duration of the album in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    /// Total number of tracks in the album
    #[serde(rename = "tracks_count")]
    pub tracks_count: Option<i32>,

    /// Number of media (disks) in the album
    #[serde(rename = "media_count")]
    pub media_count: Option<i32>,

    /// Unix timestamp of when the album was released
    #[serde(rename = "released_at")]
    pub released_at: Option<i64>,

    /// Date when the album became available for download
    #[serde(rename = "release_date_download")]
    pub release_date_download: Option<String>,

    /// Original release date of the album
    #[serde(rename = "release_date_original")]
    pub release_date_original: Option<String>,

    /// Date when the album became available for streaming
    #[serde(rename = "release_date_stream")]
    pub release_date_stream: Option<String>,

    /// Unix timestamp of when the album was created in the system
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,

    /// Unix timestamp of when the album became purchasable
    #[serde(rename = "purchasable_at")]
    pub purchasable_at: Option<i64>,

    /// Unix timestamp of when the album became streamable
    #[serde(rename = "streamable_at")]
    pub streamable_at: Option<i64>,

    /// Copyright information for the album
    #[serde(rename = "copyright")]
    pub copyright: Option<String>,

    /// Description of the album
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Catchline or tagline for the album
    #[serde(rename = "catchline")]
    pub catchline: Option<String>,

    /// Recording information for the album
    #[serde(rename = "recording_information")]
    pub recording_information: Option<String>,

    /// Maximum bit depth of the album's audio files
    #[serde(rename = "maximum_bit_depth")]
    pub maximum_bit_depth: Option<f64>,

    /// Maximum number of audio channels in the album's files
    #[serde(rename = "maximum_channel_count")]
    pub maximum_channel_count: Option<f64>,

    /// Maximum sampling rate of the album's audio files
    #[serde(rename = "maximum_sampling_rate")]
    pub maximum_sampling_rate: Option<f64>,

    /// Maximum technical specifications for the album
    #[serde(rename = "maximum_technical_specifications")]
    pub maximum_technical_specifications: Option<String>,

    /// Whether the album is available in high-resolution format
    #[serde(rename = "hires")]
    pub hires: Option<bool>,

    /// Whether the album is streamable in high-resolution format
    #[serde(rename = "hires_streamable")]
    pub hires_streamable: Option<bool>,

    /// Whether the album is displayable to users
    #[serde(rename = "displayable")]
    pub displayable: Option<bool>,

    /// Whether the album is available for download
    #[serde(rename = "downloadable")]
    pub downloadable: Option<bool>,

    /// Whether the album is available for purchase
    #[serde(rename = "purchasable")]
    pub purchasable: Option<bool>,

    /// Whether the album is available for streaming
    #[serde(rename = "streamable")]
    pub streamable: Option<bool>,

    /// Whether the album has preview tracks available
    #[serde(rename = "previewable")]
    pub previewable: Option<bool>,

    /// Whether the album has sample tracks available
    #[serde(rename = "sampleable")]
    pub sampleable: Option<bool>,

    /// Whether the album has parental content warnings
    #[serde(rename = "parental_warning")]
    pub parental_warning: Option<bool>,

    /// Whether the album is an official release
    #[serde(rename = "is_official")]
    pub is_official: Option<bool>,

    /// Type of product (e.g., "album", "single", "compilation")
    #[serde(rename = "product_type")]
    pub product_type: Option<String>,

    /// Type of release (e.g., "album", "single", "ep")
    #[serde(rename = "release_type")]
    pub release_type: Option<String>,

    /// Popularity score for the album
    #[serde(rename = "popularity")]
    pub popularity: Option<i32>,

    /// Search results for tracks in the album
    #[serde(rename = "tracks")]
    pub tracks: Option<ItemSearchResult<Box<Track>>>,

    /// Albums by the same artist
    #[serde(rename = "albums_same_artist")]
    pub albums_same_artist: Option<AlbumsSameArtist>,

    /// Area information for the album
    #[serde(rename = "area")]
    pub area: Option<Area>,

    /// Articles related to the album
    #[serde(rename = "articles")]
    pub articles: Option<Vec<Article>>,

    /// Awards received by the album
    #[serde(rename = "awards")]
    pub awards: Option<Vec<Award>>,

    /// Goodies or bonus content related to the album
    #[serde(rename = "goodies")]
    pub goodies: Option<Vec<Goody>>,

    /// Focus items related to the album
    #[serde(rename = "items_focus")]
    pub items_focus: Option<Vec<Focus>>,

    /// Period information for the album
    #[serde(rename = "period")]
    pub period: Option<Period>,

    /// Monthly product sales factor for the album
    #[serde(rename = "product_sales_factors_monthly")]
    pub product_sales_factors_monthly: Option<f64>,

    /// Weekly product sales factor for the album
    #[serde(rename = "product_sales_factors_weekly")]
    pub product_sales_factors_weekly: Option<f64>,

    /// Yearly product sales factor for the album
    #[serde(rename = "product_sales_factors_yearly")]
    pub product_sales_factors_yearly: Option<f64>,

    /// Qobuz-specific ID for the album
    #[serde(rename = "qobuz_id")]
    pub qobuz_id: Option<i32>,

    /// Release tags associated with the album
    #[serde(rename = "release_tags")]
    pub release_tags: Option<Vec<Value>>,

    /// List of track IDs included in the album
    #[serde(rename = "track_ids")]
    pub track_ids: Option<Vec<i32>>,
}

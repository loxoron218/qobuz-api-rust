use serde::{Deserialize, Serialize};

use crate::models::Image;

/// File URL model containing information about a downloadable file
///
/// This struct represents a file URL with information about the track, format,
/// and availability for download or streaming.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::FileUrl;
///
/// let file_url = FileUrl {
///     track_id: Some(12345),
///     url: Some("https://example.com/file.mp3".to_string()),
///     format_id: Some(5),
///     mime_type: Some("audio/mpeg".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FileUrl {
    /// ID of the track associated with the file
    #[serde(rename = "track_id")]
    pub track_id: Option<i32>,

    /// Duration of the track in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i32>,

    /// URL to download or stream the file
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// Format ID for the file
    #[serde(rename = "format_id")]
    pub format_id: Option<i32>,

    /// MIME type of the file
    #[serde(rename = "mime_type")]
    pub mime_type: Option<String>,

    /// Sampling rate of the file in kHz
    #[serde(rename = "sampling_rate")]
    pub sampling_rate: Option<f64>,

    /// Bit depth of the file
    #[serde(rename = "bit_depth")]
    pub bit_depth: Option<i32>,

    /// Status of the file URL (e.g., "available", "error")
    #[serde(rename = "status")]
    pub status: Option<String>,

    /// Message providing additional information about the file URL
    #[serde(rename = "message")]
    pub message: Option<String>,

    /// Code providing additional information about the file URL
    #[serde(rename = "code")]
    pub code: Option<String>,
}

/// Releases list model containing a list of releases
///
/// This struct represents a paginated list of releases with information about
/// whether more releases are available.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{ReleasesList, Release};
///
/// let releases_list = ReleasesList {
///     has_more: Some(true),
///     items: Some(vec![Release::default()]),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleasesList {
    /// Whether there are more releases available beyond the current list
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,

    /// List of releases in the current page
    #[serde(rename = "items")]
    pub items: Option<Vec<Release>>,
}

/// Release model containing information about a music release
///
/// This struct represents a music release with details about the album, artist,
/// image, UPC, release date, label, and various other properties.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Release;
///
/// let release = Release {
///     id: Some("release123".to_string()),
///     title: Some("Example Release".to_string()),
///     release_date: Some("2023-01-01".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Release {
    /// Unique identifier for the release
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Title of the release
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Artist information for the release
    #[serde(rename = "artist")]
    pub artist: Option<ReleaseArtist>,

    /// Image information for the release artwork
    #[serde(rename = "image")]
    pub image: Option<Image>,

    /// Universal Product Code for the release
    #[serde(rename = "upc")]
    pub upc: Option<String>,

    /// Release date of the release
    #[serde(rename = "release_date")]
    pub release_date: Option<String>,

    /// Label that released the album
    #[serde(rename = "label")]
    pub label: Option<String>,

    /// Version information for the release
    #[serde(rename = "version")]
    pub version: Option<String>,

    /// Number of tracks in the release
    #[serde(rename = "tracks_count")]
    pub tracks_count: Option<i32>,

    /// Duration of the release in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    /// Copyright information for the release
    #[serde(rename = "copyright")]
    pub copyright: Option<String>,

    /// URL to the release on Qobuz
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// Whether the release is high-quality
    #[serde(rename = "is_hq")]
    pub is_hq: Option<bool>,

    /// Whether the release has explicit content
    #[serde(rename = "is_explicit")]
    pub is_explicit: Option<bool>,

    /// List of tracks in the release
    #[serde(rename = "tracks")]
    pub tracks: Option<ReleaseTrackList>,

    /// Physical support information for the release
    #[serde(rename = "physical_support")]
    pub physical_support: Option<ReleasePhysicalSupport>,

    /// Rights information for the release
    #[serde(rename = "rights")]
    pub rights: Option<ReleaseRights>,

    /// Audio information for the release
    #[serde(rename = "audio_info")]
    pub audio_info: Option<ReleaseAudioInfo>,
}

/// Release artist model containing information about an artist in a release
///
/// This struct represents an artist associated with a release, including their
/// identification, name, and slug.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ReleaseArtist;
///
/// let release_artist = ReleaseArtist {
///     id: Some(12345),
///     name: Some("Example Artist".to_string()),
///     slug: Some("example-artist".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleaseArtist {
    /// Unique identifier for the artist
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the artist
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the artist
    #[serde(rename = "slug")]
    pub slug: Option<String>,
}

/// Release track list model containing a list of tracks in a release
///
/// This struct represents a list of tracks in a release along with the total count.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{ReleaseTrackList, ReleaseTrack};
///
/// let track_list = ReleaseTrackList {
///     items: Some(vec![ReleaseTrack::default()]),
///     total: Some(10),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleaseTrackList {
    /// List of tracks in the release
    #[serde(rename = "items")]
    pub items: Option<Vec<ReleaseTrack>>,

    /// Total number of tracks in the release
    #[serde(rename = "total")]
    pub total: Option<i32>,
}

/// Release track model containing information about a track in a release
///
/// This struct represents a track in a release with details about its identification,
/// title, duration, and position in the release.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ReleaseTrack;
///
/// let release_track = ReleaseTrack {
///     id: Some(12345),
///     title: Some("Example Track".to_string()),
///     duration: Some(180),
///     track_number: Some(1),
///     media_number: Some(1),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleaseTrack {
    /// Unique identifier for the track
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Title of the track
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Duration of the track in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    /// Track number within the release
    #[serde(rename = "track_number")]
    pub track_number: Option<i32>,

    /// Media number (disk number) for the track
    #[serde(rename = "media_number")]
    pub media_number: Option<i32>,
}

/// Release physical support model containing information about physical media
///
/// This struct represents information about physical media support for a release,
/// including type, format, and description.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ReleasePhysicalSupport;
///
/// let physical_support = ReleasePhysicalSupport {
///     type_field: Some("cd".to_string()),
///     format: Some("CD".to_string()),
///     description: Some("Compact Disc".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleasePhysicalSupport {
    /// Type of physical support
    #[serde(rename = "type")]
    pub type_field: Option<String>,

    /// Format of the physical support
    #[serde(rename = "format")]
    pub format: Option<String>,

    /// Description of the physical support
    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// Release rights model containing information about usage rights
///
/// This struct represents the rights associated with a release, specifically
/// whether it can be streamed or downloaded.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ReleaseRights;
///
/// let rights = ReleaseRights {
///     can_stream: Some(true),
///     can_download: Some(true),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleaseRights {
    /// Whether the release can be streamed
    #[serde(rename = "can_stream")]
    pub can_stream: Option<bool>,

    /// Whether the release can be downloaded
    #[serde(rename = "can_download")]
    pub can_download: Option<bool>,
}

/// Release audio information model containing technical audio specifications
///
/// This struct represents the technical audio specifications for a release,
/// including bit depth, sampling rate, and format.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ReleaseAudioInfo;
///
/// let audio_info = ReleaseAudioInfo {
///     bit_depth: Some(24),
///     sampling_rate: Some(96.0),
///     format: Some("FLAC".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ReleaseAudioInfo {
    /// Bit depth of the audio
    #[serde(rename = "bit_depth")]
    pub bit_depth: Option<i32>,

    /// Sampling rate of the audio in kHz
    #[serde(rename = "sampling_rate")]
    pub sampling_rate: Option<f64>,

    /// Format of the audio
    #[serde(rename = "format")]
    pub format: Option<String>,
}

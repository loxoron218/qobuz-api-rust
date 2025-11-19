use serde::{Deserialize, Serialize};

use crate::models::{Album, Artist, AudioInfo};

/// Track model representing a track on the Qobuz platform
///
/// This struct contains comprehensive information about a track including its
/// identification, title, version, duration, album, artists, and various metadata.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Track;
///
/// let track = Track {
///     id: Some(12345),
///     title: Some("Example Track".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Track {
    /// Unique identifier for the track
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Title of the track
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Version information for the track (e.g., "Remastered", "Acoustic")
    #[serde(rename = "version")]
    pub version: Option<String>,

    /// International Standard Recording Code for the track
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,

    /// Track number within its album
    #[serde(rename = "track_number")]
    pub track_number: Option<i32>,

    /// Duration of the track in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    /// Media number (disk number) for the track
    #[serde(rename = "media_number")]
    pub media_number: Option<i32>,

    /// Work information for the track (for classical music)
    #[serde(rename = "work")]
    pub work: Option<String>,

    /// Album that contains this track (boxed to handle recursive structures)
    #[serde(rename = "album")]
    pub album: Option<Box<Album>>,

    /// Main performer of the track (boxed to handle recursive structures)
    #[serde(rename = "performer")]
    pub performer: Option<Box<Artist>>,

    /// List of performers for the track as a string
    #[serde(rename = "performers")]
    pub performers: Option<String>,

    /// Composer of the track (boxed to handle recursive structures)
    #[serde(rename = "composer")]
    pub composer: Option<Box<Artist>>,

    /// Audio information for the track
    #[serde(rename = "audio_info")]
    pub audio_info: Option<AudioInfo>,

    /// Copyright information for the track
    #[serde(rename = "copyright")]
    pub copyright: Option<String>,

    /// Whether the track is displayable to users
    #[serde(rename = "displayable")]
    pub displayable: Option<bool>,

    /// Whether the track is available for download
    #[serde(rename = "downloadable")]
    pub downloadable: Option<bool>,

    /// Whether the track is available for purchase
    #[serde(rename = "purchasable")]
    pub purchasable: Option<bool>,

    /// Whether the track is available for streaming
    #[serde(rename = "streamable")]
    pub streamable: Option<bool>,

    /// Whether the track has a preview available
    #[serde(rename = "previewable")]
    pub previewable: Option<bool>,

    /// Whether the track has a sample available
    #[serde(rename = "sampleable")]
    pub sampleable: Option<bool>,

    /// Whether the track is available in high-resolution format
    #[serde(rename = "hires")]
    pub hires: Option<bool>,

    /// Whether the track is streamable in high-resolution format
    #[serde(rename = "hires_streamable")]
    pub hires_streamable: Option<bool>,

    /// Maximum bit depth of the track's audio file
    #[serde(rename = "maximum_bit_depth")]
    pub maximum_bit_depth: Option<f64>,

    /// Maximum number of audio channels in the track's file
    #[serde(rename = "maximum_channel_count")]
    pub maximum_channel_count: Option<f64>,

    /// Maximum sampling rate of the track's audio file
    #[serde(rename = "maximum_sampling_rate")]
    pub maximum_sampling_rate: Option<f64>,

    /// Unix timestamp of when the track became purchasable
    #[serde(rename = "purchasable_at")]
    pub purchasable_at: Option<i64>,

    /// Unix timestamp of when the track became streamable
    #[serde(rename = "streamable_at")]
    pub streamable_at: Option<i64>,

    /// Date when the track became available for download
    #[serde(rename = "release_date_download")]
    pub release_date_download: Option<String>,

    /// Original release date of the track
    #[serde(rename = "release_date_original")]
    pub release_date_original: Option<String>,

    /// Date when the track became available for streaming
    #[serde(rename = "release_date_stream")]
    pub release_date_stream: Option<String>,

    /// Whether the track has parental content warnings
    #[serde(rename = "parental_warning")]
    pub parental_warning: Option<bool>,
}

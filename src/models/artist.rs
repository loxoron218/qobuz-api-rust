use {
    serde::{Deserialize, Serialize},
    serde_json::Value,
};

use crate::models::{Album, Biography, Image, ItemSearchResult, Playlist};

/// Artist model representing an artist on the Qobuz platform
///
/// This struct contains comprehensive information about an artist including their
/// identification, name, profile picture, album counts, roles, and related content.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Artist;
///
/// let artist = Artist {
///     id: Some(12345),
///     name: Some("Example Artist".to_string()),
///     slug: Some("example-artist".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Artist {
    /// Unique identifier for the artist
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// The name of the artist
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the artist
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// URL to the artist's profile picture
    #[serde(rename = "picture")]
    pub picture: Option<String>,

    /// Total number of albums associated with the artist
    #[serde(rename = "albums_count")]
    pub albums_count: Option<i32>,

    /// Number of albums where the artist is the primary artist
    #[serde(rename = "albums_as_primary_artist_count")]
    pub albums_as_primary_artist_count: Option<i32>,

    /// Number of albums where the artist is the primary composer
    #[serde(rename = "albums_as_primary_composer_count")]
    pub albums_as_primary_composer_count: Option<i32>,

    /// List of roles the artist has (e.g., "main", "featured")
    #[serde(rename = "roles")]
    pub roles: Option<Vec<String>>,

    /// Detailed image information for the artist
    #[serde(rename = "image")]
    pub image: Option<Image>,

    /// Biography information for the artist
    #[serde(rename = "biography")]
    pub biography: Option<Biography>,

    /// List of IDs for similar artists
    #[serde(rename = "similar_artist_ids")]
    pub similar_artist_ids: Option<Vec<i32>>,

    /// Search results for playlists associated with the artist
    #[serde(rename = "playlists")]
    pub playlists: Option<ItemSearchResult<Box<Playlist>>>,

    /// Search results for albums by the artist
    #[serde(rename = "albums")]
    pub albums: Option<ItemSearchResult<Box<Album>>>,

    /// Search results for albums by the artist, excluding the last release
    #[serde(rename = "albums_without_last_release")]
    pub albums_without_last_release: Option<ItemSearchResult<Box<Album>>>,

    /// The artist's most recent album release
    #[serde(rename = "album_last_release")]
    pub album_last_release: Option<Box<Album>>,

    /// Additional information about the artist as raw JSON value
    #[serde(rename = "information")]
    pub information: Option<Value>,
}

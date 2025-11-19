use serde::{Deserialize, Serialize};

use crate::models::{
    Artist, Focus, Genre, Image, ItemSearchResult, Playlist as PlaylistModel, Tag, Track, User,
};

/// Playlist model containing information about a user playlist
///
/// This struct represents a playlist with details about its content, owner,
/// creation date, and various properties.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Playlist;
///
/// let playlist = Playlist {
///     id: Some(789012345),
///     name: Some("My Favorites".to_string()),
///     is_public: Some(true),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Playlist {
    /// Unique identifier for the playlist
    #[serde(rename = "id")]
    pub id: Option<i64>,

    /// Name of the playlist
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Description of the playlist
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Total duration of the playlist in seconds
    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    /// Unix timestamp of when the playlist was created
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,

    /// Unix timestamp of when the playlist was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,

    /// Unix timestamp of when the playlist was made public
    #[serde(rename = "public_at")]
    pub public_at: Option<i64>,

    /// Unix timestamp marking the start of the playlist's publication period
    #[serde(rename = "published_from")]
    pub published_from: Option<i64>,

    /// Unix timestamp marking the end of the playlist's publication period
    #[serde(rename = "published_to")]
    pub published_to: Option<i64>,

    /// Number of tracks in the playlist
    #[serde(rename = "tracks_count")]
    pub tracks_count: Option<i32>,

    /// Number of users following the playlist
    #[serde(rename = "users_count")]
    pub users_count: Option<i32>,

    /// Whether the playlist is public
    #[serde(rename = "is_public")]
    pub is_public: Option<bool>,

    /// Whether the playlist is featured
    #[serde(rename = "is_featured")]
    pub is_featured: Option<bool>,

    /// Whether the playlist is published
    #[serde(rename = "is_published")]
    pub is_published: Option<bool>,

    /// Whether the playlist is collaborative
    #[serde(rename = "is_collaborative")]
    pub is_collaborative: Option<bool>,

    /// User who owns the playlist
    #[serde(rename = "owner")]
    pub owner: Option<User>,

    /// Image information for the playlist artwork
    #[serde(rename = "image")]
    pub image: Option<Image>,

    /// List of image URLs for the playlist
    #[serde(rename = "images")]
    pub images: Option<Vec<String>>,

    /// List of rectangular image URLs for the playlist
    #[serde(rename = "image_rectangle")]
    pub image_rectangle: Option<Vec<String>>,

    /// List of small rectangular image URLs for the playlist
    #[serde(rename = "image_rectangle_mini")]
    pub image_rectangle_mini: Option<Vec<String>>,

    /// List of 150px square image URLs for the playlist
    #[serde(rename = "images150")]
    pub images150: Option<Vec<String>>,

    /// List of 300px square image URLs for the playlist
    #[serde(rename = "images300")]
    pub images300: Option<Vec<String>>,

    /// Search results for tracks in the playlist
    #[serde(rename = "tracks")]
    pub tracks: Option<ItemSearchResult<Box<Track>>>,

    /// List of genres associated with the playlist
    #[serde(rename = "genres")]
    pub genres: Option<Vec<Genre>>,

    /// List of tags associated with the playlist
    #[serde(rename = "tags")]
    pub tags: Option<Vec<Tag>>,

    /// List of featured artists in the playlist
    #[serde(rename = "featured_artists")]
    pub featured_artists: Option<Vec<Box<Artist>>>,

    /// Search results for similar playlists
    #[serde(rename = "similar_playlists")]
    pub similar_playlists: Option<ItemSearchResult<Box<PlaylistModel>>>,

    /// Focus items related to the playlist
    #[serde(rename = "items_focus")]
    pub items_focus: Option<Vec<Focus>>,

    /// Timestamp position within the playlist
    #[serde(rename = "timestamp_position")]
    pub timestamp_position: Option<i64>,

    /// URL-friendly slug for the playlist
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// List of stores where the playlist is available
    #[serde(rename = "stores")]
    pub stores: Option<Vec<String>>,
}

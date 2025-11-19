use serde::{Deserialize, Serialize};

use crate::models::{
    Album, Article, Artist, MostPopular as MostPopularModel, Playlist, Story, Track, User,
};

/// Generic search result model for items
///
/// This struct represents a paginated search result containing a list of items
/// of type T, along with metadata about the total count, pagination limits, and
/// whether more results are available.
///
/// # Type Parameters
///
/// * `T` - The type of items contained in the search result
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::ItemSearchResult;
///
/// let result: ItemSearchResult<String> = ItemSearchResult {
///     items: Some(vec!["item1".to_string(), "item2".to_string()]),
///     total: Some(100),
///     limit: Some(10),
///     offset: Some(0),
///     has_more: Some(true),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemSearchResult<T> {
    /// List of items in the search result
    #[serde(rename = "items")]
    pub items: Option<Vec<T>>,

    /// Total number of items matching the search criteria
    #[serde(rename = "total")]
    pub total: Option<i32>,

    /// Maximum number of items returned per page
    #[serde(rename = "limit")]
    pub limit: Option<i32>,

    /// Offset indicating how many items were skipped
    #[serde(rename = "offset")]
    pub offset: Option<i32>,

    /// Whether there are more items available beyond the current page
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
}

// Implement Default trait for all structs to make testing easier
impl<T> Default for ItemSearchResult<T> {
    fn default() -> Self {
        ItemSearchResult {
            items: None,
            total: None,
            limit: None,
            offset: None,
            has_more: None,
        }
    }
}

/// Search result model containing results for various content types
///
/// This struct represents the results of a search operation across different
/// content types including albums, articles, artists, playlists, and tracks.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{SearchResult, ItemSearchResult, Album, Artist};
///
/// let search_result = SearchResult {
///     albums: Some(ItemSearchResult::<Album>::default()),
///     artists: Some(ItemSearchResult::<Artist>::default()),
///     query: Some("search query".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SearchResult {
    /// Search results for albums
    #[serde(rename = "albums")]
    pub albums: Option<ItemSearchResult<Album>>,

    /// Search results for articles
    #[serde(rename = "articles")]
    pub articles: Option<ItemSearchResult<Article>>,

    /// Search results for artists
    #[serde(rename = "artists")]
    pub artists: Option<ItemSearchResult<Artist>>,

    /// Search results for focus items
    #[serde(rename = "focus")]
    pub focus: Option<ItemSearchResult<Story>>,

    /// Search results for most popular items
    #[serde(rename = "most_popular")]
    pub most_popular: Option<ItemSearchResult<MostPopularModel>>,

    /// Search results for playlists
    #[serde(rename = "playlists")]
    pub playlists: Option<ItemSearchResult<Playlist>>,

    /// The search query that was used
    #[serde(rename = "query")]
    pub query: Option<String>,

    /// Search results for stories
    #[serde(rename = "stories")]
    pub stories: Option<ItemSearchResult<Story>>,

    /// Search results for tracks
    #[serde(rename = "tracks")]
    pub tracks: Option<ItemSearchResult<Track>>,
}

/// Most popular model containing information about popular content
///
/// This struct represents information about the most popular content of a specific type.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{MostPopular, MostPopularContent};
///
/// let most_popular = MostPopular {
///     content: MostPopularContent::default(),
///     type_field: Some("album".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MostPopular {
    /// Content information for the most popular item
    #[serde(rename = "content")]
    pub content: MostPopularContent,

    /// Type of the most popular content
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// Most popular content model containing type information
///
/// This struct represents the content type for most popular items.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::MostPopularContent;
///
/// let content = MostPopularContent {
///     type_field: Some("track".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MostPopularContent {
    /// Type of the content
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// User favorites model containing a user's favorite content
///
/// This struct represents a user's favorite content including albums, artists,
/// tracks, and articles, along with the user information.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{UserFavorites, ItemSearchResult, Album, Artist, Track, Article};
///
/// let user_favorites = UserFavorites {
///     user: None,
///     albums: Some(ItemSearchResult::<Album>::default()),
///     artists: Some(ItemSearchResult::<Artist>::default()),
///     tracks: Some(ItemSearchResult::<Track>::default()),
///     articles: Some(ItemSearchResult::<Article>::default()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserFavorites {
    /// User information for the favorites
    #[serde(rename = "user")]
    pub user: Option<User>,

    /// Search results for favorite albums
    #[serde(rename = "albums")]
    pub albums: Option<ItemSearchResult<Album>>,

    /// Search results for favorite artists
    #[serde(rename = "artists")]
    pub artists: Option<ItemSearchResult<Artist>>,

    /// Search results for favorite tracks
    #[serde(rename = "tracks")]
    pub tracks: Option<ItemSearchResult<Track>>,

    /// Search results for favorite articles
    #[serde(rename = "articles")]
    pub articles: Option<ItemSearchResult<Article>>,
}

/// User favorites IDs model containing IDs of a user's favorite content
///
/// This struct represents the IDs of a user's favorite content including albums,
/// articles, artists, and tracks.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::UserFavoritesIds;
///
/// let favorites_ids = UserFavoritesIds {
///     albums: Some(vec!["album1".to_string(), "album2".to_string()]),
///     artists: Some(vec![123, 456]),
///     tracks: Some(vec![789, 101112]),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UserFavoritesIds {
    /// List of favorite album IDs
    #[serde(rename = "albums")]
    pub albums: Option<Vec<String>>,

    /// List of favorite article IDs
    #[serde(rename = "articles")]
    pub articles: Option<Vec<i64>>,

    /// List of favorite artist IDs
    #[serde(rename = "artists")]
    pub artists: Option<Vec<i32>>,

    /// List of favorite track IDs
    #[serde(rename = "tracks")]
    pub tracks: Option<Vec<i32>>,
}

/// Model containing albums by the same artist
///
/// This struct represents a collection of albums by the same artist along with
/// information about the artist.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::{AlbumsSameArtist, ItemSearchResult, Album};
///
/// let albums_same_artist = AlbumsSameArtist {
///     albums: Some(ItemSearchResult::<Box<Album>>::default()),
///     artist: None,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AlbumsSameArtist {
    /// Search results for albums by the same artist
    #[serde(rename = "albums")]
    pub albums: Option<ItemSearchResult<Box<Album>>>,

    /// The artist whose albums are included
    #[serde(rename = "artist")]
    pub artist: Option<Box<Artist>>,
}

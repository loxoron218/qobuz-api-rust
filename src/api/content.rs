/// Qobuz API content modules.
///
/// This module contains various submodules that handle different types of content available
/// through the Qobuz API, including albums, artists, catalogs, labels, playlists, and tracks.
/// Each submodule provides specific functionality for interacting with the corresponding
/// content type on the Qobuz platform.
///
/// # Example
///
/// ```no_run
/// use qobuz_api_rust::QobuzApiService;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let service = QobuzApiService::new().await?;
///
///     // Search for albums
///     let albums = service.search_albums("radiohead", Some(10), None, None).await?;
///
///     // Get a specific track
///     let track = service.get_track("12345", None).await?;
///
///     Ok(())
/// }
/// ```
/// Album-related API functionality.
///
/// This module provides methods for retrieving and searching albums on the Qobuz platform.
/// It includes functionality for getting detailed album information, searching for albums
/// by title or artist, and downloading entire albums.
pub mod albums;

/// Artist-related API functionality.
///
/// This module provides methods for retrieving and searching artists on the Qobuz platform.
/// It includes functionality for getting detailed artist information, searching for artists
/// by name, and retrieving an artist's discography.
pub mod artists;

/// Catalog-related API functionality.
///
/// This module provides methods for searching the overall Qobuz catalog. It allows searching
/// across multiple content types (albums, artists, tracks, etc.) with a single query.
pub mod catalog;

/// Label and article-related API functionality.
///
/// This module provides methods for interacting with labels and articles on the Qobuz platform.
/// It includes functionality for searching articles and retrieving label information.
pub mod labels_and_articles;

/// Playlist-related API functionality.
///
/// This module provides methods for retrieving and searching playlists on the Qobuz platform.
/// It includes functionality for getting detailed playlist information and searching for
/// playlists by name or content.
pub mod playlists;

/// Track-related API functionality.
///
/// This module provides methods for retrieving, searching, and downloading tracks on the Qobuz platform.
/// It includes functionality for getting detailed track information, searching for tracks,
/// retrieving download URLs, and downloading tracks with embedded metadata.
pub mod tracks;

/// Authentication module for the Qobuz API.
///
/// This module provides functionality for authenticating with the Qobuz API,
/// including user login with credentials or tokens, and password reset capabilities.
/// It handles the management of authentication tokens required for accessing
/// protected API endpoints.
pub mod auth;

/// Content modules for the Qobuz API.
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
pub mod content;

/// Favorites management module for the Qobuz API.
///
/// This module provides functionality for managing user favorites, including adding,
/// removing, and retrieving favorite tracks, albums, and artists. It requires user
/// authentication to access and modify the user's favorites.
pub mod favorites;

/// HTTP request handling module for the Qobuz API.
///
/// This module contains the core request functions used to communicate with the Qobuz API.
/// It handles GET, POST, and signed GET requests with proper authentication, parameter
/// handling, and response parsing. The functions in this module are used by other API
/// modules to make actual HTTP calls to the Qobuz API endpoints.
pub mod requests;

/// Main service module for the Qobuz API.
///
/// This module contains the core `QobuzApiService` struct and its implementation,
/// which serves as the main interface for interacting with the Qobuz API. It handles
/// authentication, credential management, and provides methods for accessing all
/// available API endpoints through a unified interface.
pub mod service;

/// # Qobuz API Rust Client
///
/// A comprehensive Rust client library for interacting with the Qobuz music streaming API.
/// This library provides functionality for authentication, content retrieval, favorites management,
/// and metadata handling for Qobuz music content.
///
/// ## Features
///
/// - **API Access**: Full access to Qobuz API endpoints for albums, artists, tracks, playlists, and more
/// - **Authentication**: Support for both user-based and token-based authentication
/// - **Content Management**: Search, retrieve, and manage Qobuz content including albums, tracks, and playlists
/// - **Metadata Handling**: Extract and embed comprehensive metadata in audio files
/// - **Download Support**: Download tracks and albums with various quality options
/// - **Format Support**: Support for various audio formats including FLAC and MP3 with different quality levels
///
/// ## Getting Started
///
/// To use this library, you'll need to initialize the [QobuzApiService] which handles API communication
/// and authentication. The service can automatically fetch application credentials from the Qobuz
/// web player or use custom credentials.
///
/// ```no_run
/// use qobuz_api_rust::QobuzApiService;
///
/// #[tokio::main]
/// async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
///     // Initialize the service with automatic credential fetching
///     let mut service = QobuzApiService::new().await?;
///
///     // Authenticate using environment variables
///     service.authenticate_with_env().await?;
///
///     // Search for content
///     let search_results = service.search_albums("Miles Davis", Some(10), None, None).await?;
///
///     Ok(())
/// }
/// ```
///
/// ## Authentication
///
/// The library supports multiple authentication methods:
/// - Token-based authentication using user ID and auth token
/// - Email/password authentication
/// - Username/password authentication
///
/// Set the appropriate environment variables in your `.env` file:
/// - For token-based: `QOBUZ_USER_ID` and `QOBUZ_USER_AUTH_TOKEN`
/// - For email-based: `QOBUZ_EMAIL` and `QOBUZ_PASSWORD`
/// - For username-based: `QOBUZ_USERNAME` and `QOBUZ_PASSWORD`
///
/// ## Modules
///
/// - [api](api/index.html): Core API functionality and service implementation
/// - [errors](errors/index.html): Custom error types for the library
/// - [metadata](metadata/index.html): Metadata extraction and embedding utilities
/// - [models](models/index.html): Data models for API responses
/// - [utils](utils/index.html): Utility functions for common operations
///
/// ## License
///
/// This library is distributed under the GPL-3.0-or-later license.
pub mod api;
/// Error types for the Qobuz API Rust library.
///
/// This module defines custom error types that can occur when using the Qobuz API library.
/// It includes errors from API responses, network operations, parsing, and authentication.
/// All errors implement the `Error` trait and provide detailed error information for
/// proper error handling throughout the library.
pub mod errors;
/// Metadata handling utilities for audio files.
///
/// This module provides functionality for extracting and embedding metadata in audio files.
/// It includes utilities for working with various audio formats (FLAC, MP3, etc.) and
/// handling comprehensive metadata including artist information, album details, track data,
/// and cover art.
pub mod metadata {
    /// Module for embedding metadata into audio files.
    ///
    /// This module provides functionality to embed comprehensive metadata into audio files.
    /// It handles different audio formats with format-specific tagging approaches to ensure
    /// compatibility and consistency. The embedder can handle various metadata fields
    /// including track titles, artist information, album details, release dates, and cover art.
    pub mod embedder;
    /// Module for extracting metadata from Qobuz API responses.
    ///
    /// This module provides functionality to extract comprehensive metadata from Qobuz API
    /// response objects into a standardized key-value format. The extracted metadata can
    /// be used for various purposes such as embedding in audio files, displaying in
    /// applications, or processing in audio workflows.
    pub mod extractor;
}
/// Data models for Qobuz API responses.
///
/// This module contains all the data structures used to represent Qobuz API responses.
/// These models are used for deserializing JSON responses from the API into Rust structs.
/// The models cover all major Qobuz content types including albums, artists, tracks,
/// playlists, users, and various metadata fields.
pub mod models;
/// Utility functions for the Qobuz API Rust library.
///
/// This module provides various utility functions used throughout the library including
/// MD5 hashing, query string building, timestamp generation, credential management,
/// filename sanitization, and HTTP response handling. These utilities support the core
/// API functionality and provide common operations needed for working with the Qobuz API.
pub mod utils;

/// Re-exports of commonly used types and functions for convenience.
///
/// This section re-exports the most important types and functions from the library
/// to provide a convenient and streamlined API for users. These re-exports allow
/// direct access to the core functionality without having to specify full module paths.
pub use {
    api::service::QobuzApiService, // The main Qobuz API service struct that provides access to all API functionality.
    errors::QobuzApiError, // The main error type for the library that encompasses all possible errors.
    metadata::{embedder::embed_metadata_in_file, extractor::extract_comprehensive_metadata}, // Functions to embed and extract metadata in audio files using extracted Qobuz data.
    utils::download_image, // Utility function to download images from URLs.
};

/// Metadata handling utilities for Qobuz audio content.
///
/// This module provides comprehensive functionality for working with metadata from
/// Qobuz music content, including extraction from API responses and embedding into
/// audio files. The module is designed to handle various audio formats (FLAC, MP3, etc.)
/// with format-specific tagging approaches that ensure compatibility with the original
/// C# Qobuz implementation.
///
/// # Overview
///
/// The metadata module consists of three main components:
///
/// - **Configuration** ([`MetadataConfig`](config/struct.MetadataConfig.html)): Controls which metadata fields are embedded in audio files
/// - **Embedding** ([`embed_metadata_in_file`](embedder/fn.embed_metadata_in_file.html)): Embeds comprehensive metadata into audio files
/// - **Extraction** ([`extract_comprehensive_metadata`](extractor/fn.extract_comprehensive_metadata.html)): Extracts metadata from Qobuz API responses into a standardized format
///
/// # Usage Examples
///
/// ## Basic metadata embedding
///
/// ```rust,no_run
/// use qobuz_api_rust::{models::{Track, Album, Artist}, metadata::{embed_metadata_in_file, MetadataConfig}};
///
/// // Assuming you have track, album, and artist data from the Qobuz API
/// // let track: Track = /* ... */;
/// // let album: Album = /* ... */;
/// // let artist: Artist = /* ... */;
///
/// // Use default configuration (most fields enabled, comment disabled)
/// let config = MetadataConfig::default();
///
/// // Embed metadata into an audio file
/// tokio_test::block_on(async {
///     embed_metadata_in_file("path/to/audio.flac", &track, &album, &artist, &config).await.unwrap();
/// });
/// ```
///
/// ## Custom metadata configuration
///
/// ```rust
/// use qobuz_api_rust::metadata::MetadataConfig;
///
/// // Create a minimal configuration
/// let minimal_config = MetadataConfig {
///     album_artist: true,
///     artist: true,
///     track_title: true,
///     album: true,
///     track_number: true,
///     disc_number: true,
///     cover_art: true,
///     ..Default::default()
/// };
///
/// // Or disable all metadata embedding
/// let no_metadata_config = MetadataConfig {
///     album_artist: false,
///     artist: false,
///     track_title: false,
///     track_number: false,
///     track_total: false,
///     disc_number: false,
///     disc_total: false,
///     album: false,
///     explicit: false,
///     upc: false,
///     isrc: false,
///     copyright: false,
///     composer: false,
///     genre: false,
///     release_year: false,
///     release_date: false,
///     comment: false,
///     cover_art: false,
///     label: false,
///     producer: false,
///     involved_people: false,
///     url: false,
///     media_type: false,
/// };
/// ```
///
/// ## Metadata extraction for processing
///
/// ```rust
/// use qobuz_api_rust::{models::{Track, Album, Artist}, metadata::extract_comprehensive_metadata};
///
/// // Assuming you have track, album, and artist data
/// // let track: Track = /* ... */;
/// // let album: Album = /* ... */;
/// // let artist: Artist = /* ... */;
///
/// let metadata = extract_comprehensive_metadata(&track, &album, &artist);
/// if let Some(title) = metadata.get("TITLE") {
///     println!("Track title: {}", title);
/// }
/// ```
///
/// # Format Considerations
///
/// The library handles different audio formats with appropriate tagging standards:
///
/// - **FLAC**: Uses Vorbis Comments format with specific field mappings
/// - **MP3**: Uses ID3v2 format with standard frame mappings
/// - **Other formats**: Defaults to ID3v2-compatible tagging
///
/// Some metadata fields have format-specific behavior to match the original C#
/// implementation, particularly around artist naming conventions and composer handling.
///
/// # Features
///
/// - **Comprehensive metadata support**: Handles all major Qobuz metadata fields
/// - **Format-specific tagging**: Proper tag mapping for different audio formats
/// - **Cover art embedding**: Downloads and embeds highest quality available artwork
/// - **Duplicate prevention**: Intelligent deduplication of artists and composers
/// - **Classical music support**: Special handling for conductor and orchestra roles
/// - **Configurable embedding**: Fine-grained control over which fields to embed
///
/// # Re-exports
///
/// The most commonly used items are re-exported at the crate root for convenience:
/// - [`MetadataConfig`](config/struct.MetadataConfig.html)
/// - [`embed_metadata_in_file`](embedder/fn.embed_metadata_in_file.html)
/// - [`extract_comprehensive_metadata`](extractor/fn.extract_comprehensive_metadata.html)
pub mod config;
/// Audio file metadata embedding functionality.
///
/// Provides the [`embed_metadata_in_file`] function for embedding comprehensive
/// metadata from Qobuz API responses into audio files with format-specific handling.
pub mod embedder;
/// Metadata extraction from Qobuz API responses.
///
/// Provides the [`extract_comprehensive_metadata`] function for extracting
/// metadata from Qobuz API response objects into a standardized key-value format.
pub mod extractor;

pub use {
    config::MetadataConfig, embedder::embed_metadata_in_file,
    extractor::extract_comprehensive_metadata,
};

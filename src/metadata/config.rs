use serde::{Deserialize, Serialize};

/// Configuration for metadata embedding in audio files.
///
/// This struct controls which metadata tags are written to audio files during the
/// metadata embedding process. It provides fine-grained control over the embedding
/// of various metadata fields, allowing users to customize which information is
/// included in their audio files.
///
/// The configuration mirrors the options available in the original C# Qobuz
/// application, ensuring compatibility and consistent behavior across implementations.
/// Each field corresponds to a specific metadata tag that can be embedded in
/// supported audio formats (FLAC, MP3, etc.).
///
/// # Usage
///
/// The [`MetadataConfig`] is used with the [`embed_metadata_in_file`] function
/// to control which metadata fields are embedded:
///
/// ```rust
/// use qobuz_api_rust::metadata::{MetadataConfig, embed_metadata_in_file};
///
/// // Create a custom configuration
/// let mut config = MetadataConfig::default();
/// config.comment = true;  // Enable comment embedding
/// config.explicit = false; // Disable explicit content flag
///
/// // Use the configuration when embedding metadata
/// // embed_metadata_in_file("path/to/file.flac", &track, &album, &artist, &config).await?;
/// ```
///
/// # Format Considerations
///
/// The actual metadata tags written depend on the audio file format:
/// - **FLAC**: Uses Vorbis Comments format
/// - **MP3**: Uses ID3v2 format
/// - **Other formats**: Default to ID3v2-compatible tagging
///
/// Some fields may have format-specific behavior (e.g., album artist handling
/// differs between FLAC and MP3 to match the original C# implementation).
///
/// # Examples
///
/// Create a minimal configuration that only embeds essential metadata:
///
/// ```rust
/// use qobuz_api_rust::metadata::MetadataConfig;
///
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
/// ```
///
/// Disable all metadata embedding (useful for privacy or minimal file size):
///
/// ```rust
/// use qobuz_api_rust::metadata::MetadataConfig;
///
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetadataConfig {
    /// Whether to embed the album artist name.
    ///
    /// For FLAC files, this uses a single album artist name with conductor priority
    /// for classical music. For MP3 files, this combines all main artists with "/"
    /// separator.
    pub album_artist: bool,
    /// Whether to embed the track artist(s) name(s).
    ///
    /// Combines artists from performers, main artist, and album artists while
    /// avoiding duplicates. Uses ", " separator for FLAC and "/" for MP3.
    pub artist: bool,
    /// Whether to embed the track title.
    ///
    /// Includes the version information (e.g., "Remastered", "Live") if available,
    /// formatted as "Title (Version)".
    pub track_title: bool,
    /// Whether to embed the track number within the album.
    ///
    /// Corresponds to the track's position in the album track listing.
    pub track_number: bool,
    /// Whether to embed the total number of tracks in the album.
    ///
    /// Indicates the complete track count for the album.
    pub track_total: bool,
    /// Whether to embed the disc number for multi-disc albums.
    ///
    /// Also known as "media number" in Qobuz API terminology.
    pub disc_number: bool,
    /// Whether to embed the total number of discs in the album.
    ///
    /// Indicates the complete disc count for multi-disc albums.
    pub disc_total: bool,
    /// Whether to embed the album title.
    ///
    /// Includes the version information if available, formatted as "Album Title (Version)".
    pub album: bool,
    /// Whether to embed explicit content information.
    ///
    /// Indicates if the track contains explicit content (parental warning).
    pub explicit: bool,
    /// Whether to embed the Universal Product Code (UPC).
    ///
    /// A barcode identifier for the album product.
    pub upc: bool,
    /// Whether to embed the International Standard Recording Code (ISRC).
    ///
    /// A unique identifier for the specific sound recording.
    pub isrc: bool,
    /// Whether to embed copyright information.
    ///
    /// Contains the copyright notice for the track.
    pub copyright: bool,
    /// Whether to embed composer information.
    ///
    /// Combines composers from performers, track composer, and album composer
    /// while avoiding duplicates. Uses "/" separator for multiple composers.
    pub composer: bool,
    /// Whether to embed genre information.
    ///
    /// Contains the primary musical genre of the album/track.
    pub genre: bool,
    /// Whether to embed the release year.
    ///
    /// Extracted from the most relevant date field (album release dates preferred,
    /// then track release dates, then timestamp).
    pub release_year: bool,
    /// Whether to embed the full release date.
    ///
    /// For FLAC files, this uses the RecordingDate field (DATE in Vorbis Comments).
    /// For MP3 files, this uses the ReleaseDate field (TDRL in ID3v2).
    pub release_date: bool,
    /// Whether to embed comment information.
    ///
    /// **Note**: Disabled by default to match common user preferences and the
    /// original C# application defaults.
    pub comment: bool,
    /// Whether to embed album cover artwork.
    ///
    /// Downloads and embeds the highest quality available album cover image,
    /// preferring mega > extralarge > large > medium > small > thumbnail sizes.
    pub cover_art: bool,
    /// Whether to embed record label information.
    ///
    /// Contains the name of the record label that released the album.
    pub label: bool,
    /// Whether to embed producer information.
    ///
    /// **Note**: Only embedded in FLAC files (as PRODUCER Vorbis Comments field).
    /// Extracted from performers with "Producer" role.
    pub producer: bool,
    /// Whether to embed involved people information.
    ///
    /// Contains the complete performers string with names and roles
    /// (e.g., "Artist Name, MainArtist - Producer Name, Producer").
    pub involved_people: bool,
    /// Whether to embed the Qobuz product URL.
    ///
    /// Creates a commercial information URL pointing to the album's Qobuz page.
    pub url: bool,
    /// Whether to embed media type information.
    ///
    /// Uses the album's release_type if available, otherwise falls back to
    /// product_type. Common values include "album", "single", "compilation", etc.
    pub media_type: bool,
}

impl Default for MetadataConfig {
    /// Returns the default metadata configuration.
    ///
    /// The default configuration enables most metadata fields to provide
    /// comprehensive metadata embedding, matching the behavior of the original
    /// C# Qobuz application. The only exception is the `comment` field, which
    /// is disabled by default based on common user preferences.
    ///
    /// # Returns
    ///
    /// A [`MetadataConfig`] instance with all fields set to `true` except
    /// `comment` which is set to `false`.
    fn default() -> Self {
        Self {
            album_artist: true,
            artist: true,
            track_title: true,
            track_number: true,
            track_total: true,
            disc_number: true,
            disc_total: true,
            album: true,
            explicit: true,
            upc: true,
            isrc: true,
            copyright: true,
            composer: true,
            genre: true,
            release_year: true,
            release_date: true,
            comment: false, // Default to false as per C# app defaults or common preference
            cover_art: true,
            label: true,
            producer: true,
            involved_people: true,
            url: true,
            media_type: true,
        }
    }
}

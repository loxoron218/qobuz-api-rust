use std::collections::{HashMap, HashSet};

use crate::models::{Album, Artist, Track};

/// Extracts comprehensive metadata from Qobuz API response objects into a key-value map.
///
/// This function takes track, album, and artist information from the Qobuz API and
/// extracts relevant metadata fields into a standardized format. The resulting
/// HashMap contains common audio file metadata tags that can be used for various
/// purposes such as embedding in audio files, displaying in applications, or
/// processing in audio workflows.
///
/// The function handles multiple sources for composer information, deduplicates
/// entries, and follows the same logic as the tag embedding function to ensure
/// consistency across the library.
///
/// # Arguments
///
/// * `track` - A reference to the [Track] object containing track-specific metadata
/// * `album` - A reference to the [Album] object containing album-specific metadata
/// * `artist` - A reference to the [Artist] object containing primary artist information
///
/// # Returns
///
/// A [HashMap] where keys are standardized metadata field names (uppercase strings)
/// and values are the corresponding metadata values as strings.
///
/// # Example
///
/// ```rust
/// use qobuz_api_rust::{models::{Track, Album, Artist}, metadata::extractor::extract_comprehensive_metadata};
///
/// // Assuming you have track, album, and artist data
/// // let metadata = extract_comprehensive_metadata(&track, &album, &artist);
/// // let title = metadata.get("TITLE");
/// ```
pub fn extract_comprehensive_metadata(
    track: &Track,
    album: &Album,
    artist: &Artist,
) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    // Extract basic track information
    if let Some(ref title) = track.title {
        // Track title - the main name of the audio track
        metadata.insert("TITLE".to_string(), title.clone());
    }

    if let Some(ref album_title) = album.title {
        // Album title - the name of the album containing the track
        metadata.insert("ALBUM".to_string(), album_title.clone());
    }

    if let Some(ref artist_name) = artist.name {
        // Artist name - the primary performing artist of the track
        metadata.insert("ARTIST".to_string(), artist_name.clone());
    }

    // Extract performer information from track
    if let Some(ref performers) = track.performers {
        // Performer information - detailed list of performers and their roles
        metadata.insert("PERFORMER".to_string(), performers.clone());
    }

    // Combine multiple composers from different sources while preventing duplicates
    // This follows the same logic as the tag embedding function for consistency
    let mut composers = Vec::new();
    let mut composer_set = HashSet::new(); // Use a set to prevent duplicates

    // Add performer as first composer if they're typically a composer (for cases like "Kendrick Lamar")
    if let Some(ref performer) = track.performer
        && let Some(ref performer_name) = performer.name
        && !composer_set.contains(performer_name)
        && performer_name != "Various Composers"
    {
        composers.push(performer_name.clone());
        composer_set.insert(performer_name.clone());
    }

    // Add track composer if exists and different from performer
    if let Some(ref track_composer) = track.composer
        && let Some(ref composer_name) = track_composer.name
        && !composer_set.contains(composer_name)
        && composer_name != "Various Composers"
    {
        composers.push(composer_name.clone());
        composer_set.insert(composer_name.clone());
    }

    // Add album composer if different from others
    if let Some(ref album_composer) = album.composer
        && let Some(ref composer_name) = album_composer.name
        && !composer_set.contains(composer_name)
        && composer_name != "Various Composers"
    {
        composers.push(composer_name.clone());
        composer_set.insert(composer_name.clone());
    }

    // Combine all composers with "/" separator as in the C# implementation
    if !composers.is_empty() {
        let combined_composers = composers.join("/");
        // Composer information - combined list of composers for the track
        metadata.insert("COMPOSER".to_string(), combined_composers);
    }

    // Extract label information from album
    if let Some(ref album_label) = album.label
        && let Some(ref label_name) = album_label.name
    {
        // Record label - the name of the record label that released the album
        metadata.insert("LABEL".to_string(), label_name.clone());
    }

    // Extract genre information from album
    if let Some(ref genre) = album.genre
        && let Some(ref genre_name) = genre.name
    {
        // Genre - the musical genre of the track/album
        metadata.insert("GENRE".to_string(), genre_name.clone());
    }

    // Extract track number information
    if let Some(track_number) = track.track_number {
        // Track number - the sequential number of the track on the album
        metadata.insert("TRACKNUMBER".to_string(), track_number.to_string());
    }

    // Extract total tracks count from album
    if let Some(ref album_tracks_count) = album.tracks_count {
        // Total tracks - the total number of tracks on the album
        metadata.insert("TRACKTOTAL".to_string(), album_tracks_count.to_string());
    }

    // Extract disc number information from album
    if let Some(ref album_media_count) = album.media_count {
        // Disc number - the disc number for multi-disc albums
        metadata.insert("DISCNUMBER".to_string(), album_media_count.to_string());
    }

    // Extract copyright information from track
    if let Some(ref copyright) = track.copyright {
        // Copyright - the copyright information for the track
        metadata.insert("COPYRIGHT".to_string(), copyright.clone());
    }

    // Extract ISRC (International Standard Recording Code) from track
    if let Some(ref isrc) = track.isrc {
        // ISRC - International Standard Recording Code, a unique identifier for sound recordings
        metadata.insert("ISRC".to_string(), isrc.clone());
    }

    // Extract and handle various date information
    if let Some(ref release_date) = track.release_date_original {
        // Release date - the original release date of the track
        metadata.insert("DATE".to_string(), release_date.clone());
    }

    // Extract stream release date from album
    if let Some(ref release_date_stream) = album.release_date_stream {
        // Stream release date - the date when the album became available for streaming
        metadata.insert(
            "RELEASE_DATE_STREAM".to_string(),
            release_date_stream.clone(),
        );
    }

    // Extract download release date from album
    if let Some(ref release_date_download) = album.release_date_download {
        // Download release date - the date when the album became available for download
        metadata.insert(
            "RELEASE_DATE_DOWNLOAD".to_string(),
            release_date_download.clone(),
        );
    }

    // Extract additional album-specific metadata
    if let Some(ref album_subtitle) = album.subtitle {
        // Album subtitle - additional descriptive text for the album
        metadata.insert("SUBTITLE".to_string(), album_subtitle.clone());
    }

    if let Some(ref album_version) = album.version {
        // Album version - indicates if this is a special version (remaster, live, etc.)
        metadata.insert("VERSION".to_string(), album_version.clone());
    }

    // Extract UPC (Universal Product Code) from album
    if let Some(ref album_upc) = album.upc {
        // UPC - Universal Product Code, a barcode symbology used for tracking trade items
        metadata.insert("UPC".to_string(), album_upc.clone());
    }

    // Extract album description
    if let Some(ref album_description) = album.description {
        // Album description - detailed description of the album
        metadata.insert("DESCRIPTION".to_string(), album_description.clone());
    }

    // Extract technical specifications from track
    if let Some(bit_depth) = track.maximum_bit_depth {
        // Bit depth - the bit depth of the audio file (e.g., 16, 24 bits)
        metadata.insert("BIT_DEPTH".to_string(), bit_depth.to_string());
    }

    if let Some(sampling_rate) = track.maximum_sampling_rate {
        // Sampling rate - the sampling rate of the audio file in kHz (e.g., 44.1, 96 kHz)
        metadata.insert("SAMPLING_RATE".to_string(), sampling_rate.to_string());
    }

    if let Some(channel_count) = track.maximum_channel_count {
        // Channel count - the number of audio channels (e.g., 2 for stereo, 6 for 5.1 surround)
        metadata.insert("CHANNELS".to_string(), channel_count.to_string());
    }

    // Extract high-resolution audio flags
    if let Some(hires) = track.hires {
        // HiRes flag - indicates if the track is high-resolution audio (true/false)
        metadata.insert("HIRES".to_string(), hires.to_string());
    }

    if let Some(hires_streamable) = track.hires_streamable {
        // HiRes streamable flag - indicates if high-resolution version is streamable (true/false)
        metadata.insert("HIRES_STREAMABLE".to_string(), hires_streamable.to_string());
    }

    metadata
}

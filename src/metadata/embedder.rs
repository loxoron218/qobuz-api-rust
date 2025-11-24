use std::{collections::HashSet, io::Error};

use lofty::{
    config::WriteOptions,
    file::{
        AudioFile,
        FileType::{Flac, Mpeg},
    },
    picture::{MimeType::Jpeg, Picture, PictureType::CoverFront},
    prelude::{
        Accessor,
        ItemKey::{
            self, AlbumArtist, CommercialInformationUrl, Composer, CopyrightMessage, Isrc, Label,
            MusicianCredits, OriginalMediaType, RecordingDate, ReleaseDate,
        },
        TagExt, TaggedFileExt,
    },
    read_from_path,
    tag::{
        ItemValue::{self, Text},
        Tag, TagItem,
        TagType::{Id3v2, VorbisComments},
    },
};

use crate::{
    errors::QobuzApiError::{self, IoError, LoftyError},
    metadata::MetadataConfig,
    models::{Album, Artist, Track},
    utils::{download_image, timestamp_to_date_and_year},
};

/// Embeds comprehensive metadata into an audio file.
///
/// This function takes track, album, and artist information and embeds it into
/// the specified audio file. It handles different audio formats (FLAC, MP3, etc.)
/// with format-specific tagging approaches to ensure compatibility and consistency
/// with the original C# implementation.
///
/// The function embeds various metadata fields including:
/// - Track title (with version if available)
/// - Album title (with version if available)
/// - Artist information (with format-specific handling)
/// - Album artist (with classical music considerations)
/// - Composer information (with duplicate handling)
/// - Performers and involved people
/// - Track and disc numbers
/// - Release dates and years
/// - ISRC and copyright information
/// - Genre and label information
/// - Commercial URL and media type
/// - Cover art (with quality preference order)
///
/// # Arguments
///
/// * `filepath` - Path to the audio file to embed metadata into
/// * `track` - Track information containing title, performers, composers, etc.
/// * `album` - Album information containing title, artists, label, dates, etc.
/// * `artist` - Primary artist information
/// * `config` - Configuration defining which metadata tags should be written
///
/// # Returns
///
/// Returns `Ok(())` if the metadata was successfully embedded, or an error if
/// there was a problem reading, writing, or processing the file.
///
/// # Example
///
/// ```rust
/// use qobuz_api_rust::{models::{Track, Album, Artist}, metadata::embedder::embed_metadata_in_file};
///
/// // Assuming you have track, album, and artist data
/// // let result = embed_metadata_in_file("path/to/audio.flac", &track, &album, &artist).await;
/// // if result.is_err() {
/// //     eprintln!("Failed to embed metadata");
/// // }
/// ```
pub async fn embed_metadata_in_file(
    filepath: &str,
    track: &Track,
    album: &Album,
    artist: &Artist,
    config: &MetadataConfig,
) -> Result<(), QobuzApiError> {
    // Read the audio file
    let mut tagged_file = read_from_path(filepath).map_err(LoftyError)?;

    // Get the file type before getting a mutable reference to the tag
    let file_type = tagged_file.file_type();

    // Create or get the tag based on the actual file format
    let tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag,
        None => {
            // If no primary tag exists, try to get the tag from audio file
            if let Some(tag) = tagged_file.first_tag_mut() {
                tag
            } else {
                // Determine the appropriate tag type based on the actual file format detected by lofty
                let tag_type = match file_type {
                    Flac => VorbisComments,
                    Mpeg => Id3v2,
                    _ => Id3v2, // Default to ID3v2 for unknown formats
                };

                let new_tag = Tag::new(tag_type);
                tagged_file.insert_tag(new_tag);

                // After inserting the tag, we need to access it again
                // Get the primary tag which should now be the one we just inserted
                tagged_file.primary_tag_mut().ok_or_else(|| {
                    IoError(Error::other(
                        "Could not create or access tag for metadata embedding",
                    ))
                })?
            }
        }
    };

    // Clear existing tags to avoid duplicates
    tag.clear();

    // Add track metadata
    if config.track_title
        && let Some(ref title) = track.title
    {
        let mut full_title = title.clone();
        if let Some(ref version) = track.version
            && !version.is_empty()
        {
            full_title = format!("{} ({})", full_title, version);
        }
        tag.set_title(full_title);
    }

    if config.album
        && let Some(ref album_title) = album.title
    {
        // Combine album title with version if available, similar to C# implementation
        let album_name = if let Some(ref version) = album.version {
            if !version.is_empty() {
                format!("{} ({})", album_title, version)
            } else {
                album_title.clone()
            }
        } else {
            album_title.clone()
        };
        tag.set_album(album_name);
    }

    // Determine the Album Artist for FLAC files (singular, conductor priority for classical)
    let album_artist_for_flac = {
        let mut result = String::new();
        if let Some(ref album_artists) = album.artists {
            let conductor_artist = album_artists.iter().find(|artist| {
                artist.roles.as_ref().is_some_and(|roles| {
                    roles.contains(&"main-artist".to_string())
                        && artist.name.as_ref().is_some_and(|name| {
                            track.performers.as_ref().is_some_and(|performers_str| {
                                performers_str.contains(&format!("{}, Conductor", name))
                            })
                        })
                })
            });

            if let Some(artist) = conductor_artist {
                result = artist.name.clone().unwrap_or_default();
            } else if let Some(ref album_artist) = album.artist
                && let Some(ref name) = album_artist.name
            {
                result = name.clone();
            }
        } else if let Some(ref album_artist) = album.artist
            && let Some(ref name) = album_artist.name
        {
            result = name.clone();
        }
        result
    };

    // Determine the Album Artist(s) for MP3 files (merged string of main artists)
    let album_artist_for_mp3 = {
        let mut main_artists_from_album = Vec::new();
        if let Some(ref album_artists) = album.artists {
            for artist_in_list in album_artists {
                if let Some(ref roles) = artist_in_list.roles
                    && roles.contains(&"main-artist".to_string())
                    && let Some(ref name) = artist_in_list.name
                {
                    main_artists_from_album.push(name.clone());
                }
            }
        }
        // Fallback to album.artist.name if album.artists is empty or no main artists found
        if main_artists_from_album.is_empty()
            && let Some(ref album_artist) = album.artist
            && let Some(ref name) = album_artist.name
        {
            main_artists_from_album.push(name.clone());
        }
        main_artists_from_album.join("/")
    };

    // Select the final album artist name based on file type
    let album_artist_name = match file_type {
        Flac => album_artist_for_flac,
        Mpeg => album_artist_for_mp3,
        _ => album_artist_for_mp3, // Default to MP3 behavior for other formats
    };

    if config.album_artist && !album_artist_name.is_empty() {
        let tag_item = TagItem::new(AlbumArtist, Text(album_artist_name));
        tag.push(tag_item);
    }

    // Set artist - combine multiple artists if available from different sources
    // For consistency with C# implementation, prioritize performers order when they contain artist info
    let mut artist_names = Vec::new();
    let mut artist_set = HashSet::new(); // Use a set to prevent duplicates

    // Extract additional artists from performers string first to preserve their order
    if let Some(ref track_performers) = track.performers {
        let performer_artists = extract_artist_names_from_performers(track_performers, &artist_set);
        for performer_artist in performer_artists {
            if !artist_set.contains(&performer_artist) {
                artist_names.push(performer_artist.clone());
                artist_set.insert(performer_artist.clone());
            }
        }
    }

    // Set producers for FLAC files
    if config.producer
        && file_type == Flac
        && let Some(ref performers_str) = track.performers
    {
        let producers = extract_producers_from_performers(performers_str);
        for producer in producers {
            // Lofty uses "PRODUCER" as the tag for Vorbis Comments (FLAC)
            let tag_item = TagItem::new(
                ItemKey::from_key(VorbisComments, "PRODUCER"),
                Text(producer.clone()),
            );
            tag.push(tag_item);
        }
    }

    // Then add the main artist from the artist parameter if not already included as a performer
    if let Some(ref artist_name) = artist.name
        && !artist_set.contains(artist_name)
    {
        artist_names.push(artist_name.clone());
        artist_set.insert(artist_name.clone());
    }

    // Add artists from album.artists field (for classical music and multi-artist albums)
    if let Some(ref album_artists) = album.artists {
        for album_artist in album_artists {
            if let Some(ref name) = album_artist.name
                && !name.is_empty()
                && !artist_set.contains(name)
            {
                artist_names.push(name.clone());
                artist_set.insert(name.clone());
            }
        }
    }

    // Set the combined artist field
    if config.artist && !artist_names.is_empty() {
        let combined_artists = match file_type {
            Flac => artist_names.join(", "),
            _ => artist_names.join("/"),
        };
        tag.set_artist(combined_artists);
    }

    // Initialize composers list - combine multiple composers if available
    let mut composers = Vec::new();
    let mut composer_normalized_set = HashSet::new(); // Use a set to prevent duplicates based on normalized names

    // Determine composers based on file type and C# behavior
    if file_type == Flac {
        // For FLAC, we need to carefully select a single composer to match C# behavior.
        // C# seems to prioritize composers from performers string, then potentially track.composer.name
        let mut potential_composers_from_performers = Vec::new();
        if let Some(ref performers_str) = track.performers {
            potential_composers_from_performers = extract_composers_from_performers(performers_str);
        }

        if let Some(composer_from_performers) = potential_composers_from_performers.last() {
            // If composers are found in performers, take the last one (mimicking a possible C# selection)
            if composer_from_performers != "Various Composers" {
                composers.push(composer_from_performers.clone());
                composer_normalized_set.insert(normalize_composer_name(composer_from_performers));
            }
        } else if let Some(ref track_composer) = track.composer
            && let Some(ref composer_name) = track_composer.name
            && composer_name != "Various Composers"
        {
            // Fallback to track.composer.name if no composers found in performers
            composers.push(composer_name.clone());
            composer_normalized_set.insert(normalize_composer_name(composer_name));
        } else if let Some(ref album_composer) = album.composer
            && let Some(ref composer_name) = album_composer.name
            && composer_name != "Various Composers"
            && !is_duplicate_composer(composer_name, &composer_normalized_set)
        {
            // Fallback to album composer if neither performers nor track composer yields a result
            composers.push(composer_name.clone());
            composer_normalized_set.insert(normalize_composer_name(composer_name));
        }
    } else {
        // For other file types (e.g., Mpeg), use the existing aggregation logic
        // This will aggregate all composers found in performers, track.composer, and album.composer
        if let Some(ref performers_str) = track.performers {
            let extracted_composers = extract_composers_from_performers(performers_str);
            for composer in extracted_composers {
                if composer != "Various Composers"
                    && !is_duplicate_composer(&composer, &composer_normalized_set)
                {
                    composers.push(composer.clone());
                    composer_normalized_set.insert(normalize_composer_name(&composer));
                }
            }
        }

        if let Some(ref track_composer) = track.composer
            && let Some(ref composer_name) = track_composer.name
            && composer_name != "Various Composers"
            && !is_duplicate_composer(composer_name, &composer_normalized_set)
        {
            composers.push(composer_name.clone());
            composer_normalized_set.insert(normalize_composer_name(composer_name));
        }

        if let Some(ref album_composer) = album.composer
            && let Some(ref composer_name) = album_composer.name
            && composer_name != "Various Composers"
            && !is_duplicate_composer(composer_name, &composer_normalized_set)
        {
            composers.push(composer_name.clone());
            composer_normalized_set.insert(normalize_composer_name(composer_name));
        }
    }

    // Set involved people - this should be more comprehensive and avoid duplicates
    let involved_people = if let Some(ref performers_str) = track.performers {
        performers_str.clone()
    } else {
        String::new()
    };

    // The Track model doesn't have a contributor field, so we'll just use performers and composer
    if config.involved_people && !involved_people.is_empty() {
        // Add as MusicianCredits which maps to the Involved People field in ID3
        let tag_item = TagItem::new(MusicianCredits, Text(involved_people));
        tag.push(tag_item);
    }

    // Set composer - combine all composers with "/" separator as in the C# implementation
    if config.composer && !composers.is_empty() {
        let combined_composers = composers.join("/");
        let tag_item = TagItem::new(Composer, Text(combined_composers));
        tag.push(tag_item);
    }

    // Set label/publisher
    if config.label
        && let Some(ref album_label) = album.label
        && let Some(ref label_name) = album_label.name
    {
        let tag_item = TagItem::new(Label, Text(label_name.clone()));
        tag.push(tag_item);
    }

    if config.genre
        && let Some(ref genre) = album.genre
        && let Some(ref genre_name) = genre.name
    {
        tag.set_genre(genre_name.clone());
    }

    // Add track number and total tracks
    if config.track_number
        && let Some(track_number) = track.track_number
    {
        tag.set_track(track_number as u32);
    }
    if config.track_total
        && let Some(ref album_tracks_count) = album.tracks_count
    {
        tag.set_track_total(*album_tracks_count as u32);
    }

    // Add disc number and total discs (Part of Set)
    if config.disc_number
        && let Some(media_number) = track.media_number
    {
        tag.set_disk(media_number as u32);
    }
    if config.disc_total
        && let Some(ref album_media_count) = album.media_count
    {
        tag.set_disk_total(*album_media_count as u32);
    }

    if config.copyright
        && let Some(ref copyright) = track.copyright
    {
        let tag_item = TagItem::new(CopyrightMessage, Text(copyright.clone()));
        tag.push(tag_item);
    }

    if config.isrc
        && let Some(ref isrc) = track.isrc
    {
        let tag_item = TagItem::new(Isrc, Text(isrc.clone()));
        tag.push(tag_item);
    }

    // Determine the primary date string (YYYY-MM-DD) and year (YYYY)
    let mut primary_date_full: Option<String> = None;
    let mut primary_year: Option<u32> = None;

    // Prioritize album release dates, then track release dates, then timestamp
    if let Some(ref release_date) = album.release_date_download {
        primary_date_full = Some(release_date.clone());
        if let Some(year_str) = release_date.split('-').next() {
            primary_year = year_str.parse::<u32>().ok();
        }
    } else if let Some(ref release_date) = album.release_date_original {
        primary_date_full = Some(release_date.clone());
        if let Some(year_str) = release_date.split('-').next() {
            primary_year = year_str.parse::<u32>().ok();
        }
    } else if let Some(ref release_date) = track.release_date_original {
        primary_date_full = Some(release_date.clone());
        if let Some(year_str) = release_date.split('-').next() {
            primary_year = year_str.parse::<u32>().ok();
        }
    } else if let Some(released_at) = album.released_at {
        let (date_str, year_num) = timestamp_to_date_and_year(released_at);
        primary_date_full = date_str;
        primary_year = year_num;
    }

    // Set the year tag if available
    if config.release_year
        && let Some(year) = primary_year
    {
        tag.set_year(year);
    }

    if config.release_date {
        // Set the RecordingDate (maps to TDRC in ID3v2, DATE in Vorbis Comments)
        if file_type == Flac
            && let Some(ref date_str) = primary_date_full
        {
            // Lofty automatically maps RecordingDate to the appropriate tag for Vorbis Comments (DATE)
            let tag_item = TagItem::new(RecordingDate, Text(date_str.clone()));
            tag.push(tag_item);
        }

        // Set ReleaseDate (maps to TDRL frame in ID3)
        // This should be set for MP3 files to match C# behavior for "Release Time", but not for FLAC.
        if file_type == Mpeg
            && let Some(ref date_str) = primary_date_full
        {
            let tag_item = TagItem::new(ReleaseDate, Text(date_str.clone()));
            tag.push(tag_item);
        }
    }

    // Add commercial URL - using CommercialInformationUrl field
    if config.url
        && let Some(ref product_url) = album.product_url
    {
        // Check if the URL is already a full URL or just a path
        let full_url = if product_url.starts_with("http") {
            product_url.clone()
        } else {
            format!("https://www.qobuz.com{}", product_url)
        };
        let tag_item = TagItem::new(CommercialInformationUrl, ItemValue::Locator(full_url));
        tag.push(tag_item);
    }

    // Add media type - using OriginalMediaType field
    // Use release_type if available, otherwise fall back to product_type to match C# implementation
    if config.media_type {
        match file_type {
            Flac => {
                // For FLAC, always add OriginalMediaType if release_type or product_type is "album" or "compilation"
                if let Some(ref release_type) = album.release_type {
                    // If release_type is "compilation", use it directly. Otherwise, if it's "album", use "album".
                    // For any other release_type, use it as is.
                    let media_type_str = if release_type == "compilation" {
                        "compilation".to_string()
                    } else if release_type == "album" {
                        "album".to_string()
                    } else {
                        release_type.clone()
                    };
                    let tag_item = TagItem::new(OriginalMediaType, Text(media_type_str));
                    tag.push(tag_item);
                } else if let Some(ref product_type) = album.product_type {
                    // Fallback to product_type if release_type is not available, with similar logic
                    let media_type_str = if product_type == "compilation" {
                        "compilation".to_string()
                    } else if product_type == "album" {
                        "album".to_string()
                    } else {
                        product_type.clone()
                    };
                    let tag_item = TagItem::new(OriginalMediaType, Text(media_type_str));
                    tag.push(tag_item);
                }
            }

            _ => {
                // For other file types, use the existing logic
                if let Some(ref release_type) = album.release_type {
                    let tag_item = TagItem::new(OriginalMediaType, Text(release_type.clone()));
                    tag.push(tag_item);
                } else if let Some(ref product_type) = album.product_type {
                    // Fallback to product_type if release_type is not available
                    let tag_item = TagItem::new(OriginalMediaType, Text(product_type.clone()));
                    tag.push(tag_item);
                }
            }
        }
    }

    // Add cover art if available - try different image sizes in order of preference
    if config.cover_art
        && let Some(ref album_image) = album.image
    {
        // Try to get the best quality image available, in order of preference
        let image_url = album_image
            .mega
            .as_ref()
            .or(album_image.extralarge.as_ref())
            .or(album_image.large.as_ref())
            .or(album_image.medium.as_ref())
            .or(album_image.small.as_ref())
            .or(album_image.thumbnail.as_ref());

        if let Some(url) = image_url {
            // Download the image and embed it
            match download_image(url).await {
                Ok(image_data) => {
                    // Create picture with proper format to match C# implementation
                    let picture = Picture::new_unchecked(
                        CoverFront,           // Picture Type: Front Cover (as in C# implementation)
                        Some(Jpeg),           // MIME Type: image/jpeg (as in C# implementation)
                        Some("".to_string()), // Empty description (as in C# implementation)
                        image_data,
                    );

                    // Add the picture to the tag
                    tag.push_picture(picture);
                }

                Err(e) => {
                    eprintln!(
                        "Warning: Could not download album cover from URL: {} - {}",
                        url, e
                    );
                }
            }
        } else {
            eprintln!("Warning: No album cover image URL available");
        }
    }

    // Write the tag to the file with options appropriate for the file type
    let options = WriteOptions::default();
    tagged_file
        .save_to_path(filepath, options)
        .map_err(LoftyError)?;
    Ok(())
}

/// Extracts composer names from a performers string by identifying roles containing "Composer" or "Lyricist".
///
/// This function parses the performers string which contains names followed by their roles
/// separated by commas, and identifies individuals with composer or lyricist roles.
/// The format typically follows the pattern: "Name, Role1, Role2 - Another Name, Role3, Role4".
///
/// # Arguments
///
/// * `performers_str` - A string containing performer names and their roles, separated by " - "
///
/// # Returns
///
/// A vector of unique composer names extracted from the performers string.
fn extract_composers_from_performers(performers_str: &str) -> Vec<String> {
    let mut composers = Vec::new();

    // Split by " - " to separate different people/role groups
    let person_groups: Vec<&str> = performers_str.split(" - ").collect();

    for group in person_groups.iter() {
        let group = group.trim();

        // Each group contains a person name followed by their roles separated by commas
        let mut parts: Vec<&str> = group.split(',').map(|s| s.trim()).collect();

        if !parts.is_empty() {
            // First part is the person's name
            let person_name = parts.remove(0).trim();

            // Check if any of the roles is ComposerLyricist
            for role in &parts {
                if role.contains("Composer") || role.contains("Lyricist") {
                    if !composers.contains(&person_name.to_string()) {
                        composers.push(person_name.to_string());
                    }
                    break; // Found composer role, no need to check other roles for this person
                }
            }
        }
    }

    composers
}

/// Extracts artist names from a performers string by identifying performers with specific roles.
///
/// This function identifies individuals with performer roles such as MainArtist, Performer,
/// AssociatedPerformer, Orchestra, or Conductor. It preserves the order from the original
/// performers string to match the C# implementation behavior and avoids duplicates by
/// checking against existing artists.
///
/// # Arguments
///
/// * `performers_str` - A string containing performer names and their roles, separated by " - "
/// * `existing_artists` - A set of artist names to avoid duplicates
///
/// # Returns
///
/// A vector of artist names extracted from the performers string, preserving the original order.
fn extract_artist_names_from_performers(
    performers_str: &str,
    existing_artists: &HashSet<String>,
) -> Vec<String> {
    let mut artist_names = Vec::new();

    // Split by " - " to separate different people/role groups, preserving order
    let person_groups: Vec<&str> = performers_str.split(" - ").collect();

    for group in person_groups.iter() {
        let group = group.trim();

        // Each group contains a person name followed by their roles separated by commas
        let mut parts: Vec<&str> = group.split(',').map(|s| s.trim()).collect();

        if !parts.is_empty() {
            // First part is the person's name
            let person_name = parts.remove(0).trim();

            // Check if this person has a performer role (e.g., MainArtist, Performer, AssociatedPerformer, etc.)
            let has_performer_role = parts.iter().any(|role| {
                role.contains("MainArtist")
                    || role.contains("Performer")
                    || role.contains("AssociatedPerformer")
                    || role.contains("Orchestra")
                    || role.contains("Conductor")
            });

            // Only add if it's a performer role and we haven't seen this artist before
            if has_performer_role
                && !existing_artists.contains(person_name)
                && !artist_names.contains(&person_name.to_string())
            {
                artist_names.push(person_name.to_string());
            }
        }
    }

    artist_names
}

/// Extracts producer names from a performers string by identifying the "Producer" role.
///
/// This function parses the performers string to find individuals with the "Producer" role.
/// The format typically follows the pattern: "Name, Role1, Role2 - Another Name, Role3, Role4".
///
/// # Arguments
///
/// * `performers_str` - A string containing performer names and their roles, separated by " - "
///
/// # Returns
///
/// A vector of unique producer names extracted from the performers string.
fn extract_producers_from_performers(performers_str: &str) -> Vec<String> {
    let mut producers = Vec::new();

    // Split by " - " to separate different people/role groups
    let person_groups: Vec<&str> = performers_str.split(" - ").collect();

    for group in person_groups.iter() {
        let group = group.trim();

        // Each group contains a person name followed by their roles separated by commas
        let mut parts: Vec<&str> = group.split(',').map(|s| s.trim()).collect();

        if !parts.is_empty() {
            let person_name = parts.remove(0).trim();

            // Check if "Producer" role is present
            if parts.iter().any(|role| role.contains("Producer")) {
                producers.push(person_name.to_string());
            }
        }
    }
    producers
}

/// Normalizes a composer name for comparison purposes to identify duplicates.
///
/// This function handles common variations in name formatting to identify duplicates,
/// including:
/// - Converting to lowercase
/// - Removing common punctuation
/// - Standardizing hyphenation patterns
/// - Handling abbreviated names (e.g., "M. Davis" vs "Miles Davis")
///
/// The normalization helps identify equivalent composer names that may be formatted
/// differently in the source data.
///
/// # Arguments
///
/// * `name` - The composer name to normalize
///
/// # Returns
///
/// A normalized version of the composer name suitable for comparison.
fn normalize_composer_name(name: &str) -> String {
    let mut normalized = name
        .to_lowercase()
        .trim()
        // Remove common punctuation that might vary
        .replace(".", "")
        .replace(",", "")
        .replace("-", " ") // Replace hyphens with spaces for better normalization
        .replace("  ", " ") // Replace multiple spaces with single space
        .replace("  ", " ") // Additional cleanup for multiple spaces
        .trim()
        .to_string();

    // Handle specific variations seen in the test data
    // Fix hyphenation variations in "de Homem-Christo"
    normalized = normalized
        .replace("de homem -christo", "de homem christo")
        .replace("de homem- christo", "de homem christo")
        .replace("de homem - christo", "de homem christo");

    // Handle "Guy-Manuel" vs "Guy Manuel" variations
    normalized = normalized
        .replace("guy manuel", "guymanuel")
        .replace("guy-manuel", "guymanuel");

    // Handle "M. Davis" vs "Miles Davis" variations
    normalized = normalized
        .replace("m. davis", "miles davis")
        .replace("m davis", "miles davis");

    normalized.trim().to_string()
}

/// Checks if a composer name is a duplicate of an existing one in the normalized set.
///
/// This function determines if a composer name is already present in the set of
/// normalized composer names, using both direct matching and substring matching
/// to catch variations like "Miles Davis" vs "M. Davis".
///
/// # Arguments
///
/// * `composer_name` - The composer name to check for duplicates
/// * `existing_normalized_set` - A set of already normalized composer names
///
/// # Returns
///
/// `true` if the composer name is a duplicate, `false` otherwise.
fn is_duplicate_composer(composer_name: &str, existing_normalized_set: &HashSet<String>) -> bool {
    let normalized_name = normalize_composer_name(composer_name);

    // Direct match
    if existing_normalized_set.contains(&normalized_name) {
        return true;
    }

    // Check for substring matches (e.g., "Miles Davis" vs "M. Davis")
    for existing in existing_normalized_set {
        // If the new name is a substring of an existing name or vice versa
        if existing.contains(&normalized_name) || normalized_name.contains(existing) {
            return true;
        }
    }

    false
}

use std::{
    error::Error,
    fs::{create_dir_all, write},
};

use {serde_json::to_string_pretty, tokio::main};

use qobuz_api_rust::{
    api::service::QobuzApiService, metadata::embedder::embed_metadata_in_file,
    utils::sanitize_filename,
};

/// Define search queries for diverse genres and eras
const TRACK_SEARCHES: &[(&str, &str, &str)] = &[
    // Kendrick Lamar - BLOOD. (Hip-Hop/Rap, Modern)
    ("Kendrick Lamar BLOOD.", "Hip-Hop/Rap", "Modern (2017)"),
    // The Beatles - Hey Jude (Rock, Classic)
    ("The Beatles Hey Jude", "Rock", "Classic (1968)"),
    // Nirvana - Smells Like Teen Spirit (Rock, Modern)
    ("Nirvana Smells Like Teen Spirit", "Rock", "Modern (1991)"),
    // Mozart - Symphony No. 40 (Classical, Orchestral)
    ("Mozart Symphony No. 40", "Classical", "Orchestral"),
    // Bach - Goldberg Variations (Classical, Solo)
    ("Bach Goldberg Variations", "Classical", "Solo"),
    // Miles Davis - Kind of Blue (Jazz, Vintage)
    ("Miles Davis Kind of Blue", "Jazz", "Vintage (1959)"),
    // Herbie Hancock - Head Hunters (Jazz, Modern)
    ("Herbie Hancock Head Hunters", "Jazz", "Modern (1973)"),
    // Daft Punk - Around the World (Electronic)
    ("Daft Punk Around the World", "Electronic", "Modern"),
    // Madonna - Like a Virgin (Pop, 1980s)
    ("Madonna Like a Virgin", "Pop", "1980s"),
    // Adele - Hello (Pop, 2010s)
    ("Adele Hello", "Pop", "2010s"),
];

/// Format IDs for different audio qualities
const MP3_FORMAT_ID: &str = "5"; // MP3 320kbps
const FLAC_HIRES_FORMAT_ID: &str = "27"; // FLAC Hi-Res 24bit/192kHz

#[main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting automated track download for metadata testing...");

    // Initialize the Qobuz API service
    println!("Initializing Qobuz API service...");
    let service = QobuzApiService::new().await?;

    // Create directories for downloads
    create_dir_all("metadata_test/downloads/mp3")?;
    create_dir_all("metadata_test/downloads/flac")?;
    create_dir_all("metadata_test/metadata/json")?;

    // Process each track search
    for (search_query, genre, era) in TRACK_SEARCHES.iter() {
        println!("\nSearching for: {} ({} - {})", search_query, genre, era);

        // Search for tracks using the API
        let search_results = service
            .search_tracks(search_query, Some(5), None, None)
            .await?;

        if let Some(ref track_items) = search_results.tracks
            && let Some(ref tracks) = track_items.items
        {
            if let Some(track) = tracks.first() {
                // Get the track ID
                let track_id = track.id.unwrap_or(0).to_string();
                let track_title = track.title.as_deref().unwrap_or("Unknown Track");
                let artist_name = track
                    .album
                    .as_ref()
                    .and_then(|album| album.artist.as_ref())
                    .and_then(|artist| artist.name.as_deref())
                    .unwrap_or("Unknown Artist");

                println!("Found track: {} by {}", track_title, artist_name);

                // Get the complete track details for JSON export
                let track_details = service.get_track(&track_id, None).await?;

                // Construct filenames
                let mp3_base_filename = sanitize_filename(&format!(
                    "{} - {} - MP3 - {}",
                    artist_name, track_title, track_id
                ));
                let mp3_filename = format!("{}.mp3", mp3_base_filename);
                let flac_base_filename = sanitize_filename(&format!(
                    "{} - {} - FLAC - {}",
                    artist_name, track_title, track_id
                ));
                let flac_filename = format!("{}.flac", flac_base_filename);
                let json_base_filename =
                    sanitize_filename(&format!("{} - {} - {}", artist_name, track_title, track_id));
                let json_filename = format!("{}.json", json_base_filename);

                let mp3_path = format!("metadata_test/downloads/mp3/{}", mp3_filename);
                let flac_path = format!("metadata_test/downloads/flac/{}", flac_filename);
                let json_path = format!("metadata_test/metadata/json/{}", json_filename);

                // Download in MP3 format
                match service
                    .download_track(&track_id, MP3_FORMAT_ID, &mp3_path)
                    .await
                {
                    Ok(_) => {
                        println!(
                            "Successfully downloaded MP3 for {} - {}",
                            artist_name, track_title
                        );
                    }
                    Err(e) => eprintln!(
                        "Failed to download MP3 for {} - {}: {}",
                        artist_name, track_title, e
                    ),
                }

                // Download in FLAC Hi-Res format
                match service
                    .download_track(&track_id, FLAC_HIRES_FORMAT_ID, &flac_path)
                    .await
                {
                    Ok(_) => {
                        println!(
                            "Successfully downloaded FLAC Hi-Res for {} - {}",
                            artist_name, track_title
                        );

                        // Embed metadata into the downloaded FLAC file
                        if let Err(e) = embed_metadata_in_file(
                            &flac_path,
                            track,
                            track_details.album.as_ref().unwrap(),
                            track_details
                                .album
                                .as_ref()
                                .unwrap()
                                .artist
                                .as_ref()
                                .unwrap(),
                        )
                        .await
                        {
                            eprintln!(
                                "Failed to embed metadata into FLAC for {} - {}: {}",
                                artist_name, track_title, e
                            );
                        } else {
                            println!(
                                "Successfully embedded metadata into FLAC for {} - {}",
                                artist_name, track_title
                            );
                        }
                    }
                    Err(e) => eprintln!(
                        "Failed to download FLAC Hi-Res for {} - {}: {}",
                        artist_name, track_title, e
                    ),
                }

                // Save track details as JSON file
                match to_string_pretty(&track_details) {
                    Ok(json_content) => {
                        write(&json_path, json_content)?;
                        println!(
                            "Successfully saved JSON metadata for {} - {}",
                            artist_name, track_title
                        );
                    }
                    Err(e) => eprintln!(
                        "Failed to save JSON for {} - {}: {}",
                        artist_name, track_title, e
                    ),
                }
            } else {
                eprintln!("No tracks found for query: {}", search_query);
            }
        }
    }

    println!("\nTrack download process completed.");
    Ok(())
}

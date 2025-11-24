use std::io::stdin;

use {dotenvy::dotenv, tokio::main};

use qobuz_api_rust::{
    api::service::QobuzApiService, errors::QobuzApiError, metadata::MetadataConfig,
    utils::sanitize_filename,
};

/// The main entry point for the Qobuz API Rust Client CLI application.
///
/// This function initializes the Qobuz API service, authenticates the user,
/// and provides an interactive loop for searching and downloading music content.
///
/// # Errors
///
/// Returns a `QobuzApiError` if there are issues with API initialization,
/// authentication, or during the search/download operations.
///
/// # Example
///
/// ```no_run
/// // This is the main function that runs when the binary is executed
/// // No direct usage in other code - it's the entry point for the CLI
/// ```
#[main]
async fn main() -> Result<(), QobuzApiError> {
    println!("Qobuz API Rust Client");

    // Load environment variables from .env file
    if let Err(e) = dotenv() {
        println!("Failed to load .env file: {}", e);
    }

    // Initialize the Qobuz API service with dynamic credentials from web player
    let mut service = QobuzApiService::new().await?;
    println!(
        "Qobuz API service initialized with app ID: {}",
        service.app_id
    );

    let config = MetadataConfig::default();

    // Try to authenticate using environment variables
    // This will automatically try different authentication methods based on available environment variables:
    // 1. QOBUZ_USER_ID and QOBUZ_USER_AUTH_TOKEN (token-based)
    // 2. QOBUZ_EMAIL and QOBUZ_PASSWORD (email/password)
    // 3. QOBUZ_USERNAME and QOBUZ_PASSWORD (username/password)
    match service.authenticate_with_env().await {
        Ok(login_result) => {
            println!("Authentication successful!");
            if let Some(user) = &login_result.user
                && let Some(user_id) = user.id
            {
                println!("User ID: {}", user_id);
            }
            if let Some(auth_token) = &login_result.auth_token {
                println!("Auth token: {}", auth_token);
            }
        }

        Err(e) => {
            println!("Authentication failed: {}", e);
            println!("Please set authentication credentials in your .env file:");
            println!("  - For token-based: QOBUZ_USER_ID and QOBUZ_USER_AUTH_TOKEN");
            println!("  - For email-based: QOBUZ_EMAIL and QOBUZ_PASSWORD");
            println!("  - For username-based: QOBUZ_USERNAME and QOBUZ_PASSWORD");
        }
    }

    loop {
        println!("\nWhat do you want to search for? (e.g., 'Miles Davis')");
        let mut query = String::new();
        stdin().read_line(&mut query).expect("Failed to read line");
        let query = query.trim();

        println!();
        println!("Search for an a) album or t) track?");
        let mut search_type = String::new();
        stdin()
            .read_line(&mut search_type)
            .expect("Failed to read line");
        let search_type = search_type.trim();

        if search_type == "a" {
            match service.search_albums(query, Some(10), None, None).await {
                Ok(result) => {
                    if let Some(albums) = result.albums
                        && let Some(items) = albums.items
                    {
                        println!();
                        println!("Found {} albums:", items.len());
                        for (i, album) in items.iter().enumerate() {
                            println!(
                                "{}) {} - {}",
                                i + 1,
                                album.artist.as_ref().map_or("Unknown Artist", |a| a
                                    .name
                                    .as_deref()
                                    .unwrap_or("Unknown Artist")),
                                album.title.as_deref().unwrap_or("No title")
                            );
                        }

                        println!("\nEnter the number of the album to download (or 'c' to cancel):");
                        let mut choice = String::new();
                        stdin().read_line(&mut choice).expect("Failed to read line");
                        let choice = choice.trim();

                        if choice == "c" {
                            continue;
                        }

                        if let Ok(album_index) = choice.parse::<usize>()
                            && album_index > 0
                            && album_index <= items.len()
                        {
                            let selected_album = &items[album_index - 1];
                            if let Some(album_id) = &selected_album.id {
                                let quality = choose_quality()?;

                                // Use the same artist/album naming structure as in download_album
                                let album_artist_name =
                                    if let Some(ref album_artist) = selected_album.artist {
                                        album_artist
                                            .name
                                            .as_ref()
                                            .unwrap_or(&"Unknown Artist".to_string())
                                            .clone()
                                    } else {
                                        "Unknown Artist".to_string()
                                    };

                                let album_title = selected_album
                                    .title
                                    .as_ref()
                                    .unwrap_or(&"Unknown Album".to_string())
                                    .clone();

                                // Create the directory structure: [Albuminterpret]/[Album]
                                let album_artist_dir = sanitize_filename(&album_artist_name);
                                let album_title_dir = sanitize_filename(&album_title);
                                let album_path =
                                    format!("downloads/{}/{}", album_artist_dir, album_title_dir);

                                println!();
                                println!("Downloading album...");
                                match service
                                    .download_album(album_id, &quality, &album_path, &config)
                                    .await
                                {
                                    Ok(_) => println!("Album downloaded successfully!"),
                                    Err(e) => println!("Failed to download album: {}", e),
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!();
                    println!("Search failed: {}", e)
                }
            }
        } else if search_type == "t" {
            match service.search_tracks(query, Some(10), None, None).await {
                Ok(result) => {
                    if let Some(tracks) = result.tracks
                        && let Some(items) = tracks.items
                    {
                        println!();
                        println!("Found {} tracks:", items.len());
                        for (i, track) in items.iter().enumerate() {
                            println!(
                                "{}) {} - {}",
                                i + 1,
                                track.performer.as_ref().map_or("Unknown Artist", |a| a
                                    .name
                                    .as_deref()
                                    .unwrap_or("Unknown Artist")),
                                track.title.as_deref().unwrap_or("No title")
                            );
                        }

                        println!("\nEnter the number of the track to download (or 'c' to cancel):");
                        let mut choice = String::new();
                        stdin().read_line(&mut choice).expect("Failed to read line");
                        let choice = choice.trim();

                        if choice == "c" {
                            continue;
                        }

                        if let Ok(track_index) = choice.parse::<usize>()
                            && track_index > 0
                            && track_index <= items.len()
                        {
                            let selected_track = &items[track_index - 1];
                            if let Some(track_id) = selected_track.id {
                                let quality = choose_quality()?;
                                let extension = match quality.as_str() {
                                    "5" => "mp3",               // MP3 320
                                    "6" | "7" | "27" => "flac", // FLAC formats
                                    _ => "flac",                // default to flac
                                };

                                // Get the track details to create proper naming
                                let track_details =
                                    service.get_track(&track_id.to_string(), None).await?;

                                // Get album details for artist and album info
                                let album_details = if let Some(ref track_album) =
                                    track_details.album
                                {
                                    track_album.as_ref().clone()
                                } else {
                                    // If no album info available, use basic naming
                                    println!("Warning: No album information available for track");
                                    let filename = format!(
                                        "downloads/{}.{}",
                                        track_details.title.as_deref().unwrap_or("track"),
                                        extension
                                    );
                                    match service
                                        .download_track(
                                            &track_id.to_string(),
                                            &quality,
                                            &filename,
                                            &config,
                                        )
                                        .await
                                    {
                                        Ok(_) => {
                                            println!();
                                            println!("Track downloaded successfully!");
                                        }
                                        Err(e) => {
                                            println!();
                                            println!("Failed to download track: {}", e)
                                        }
                                    }
                                    continue;
                                };

                                // Get album artist name
                                let album_artist_name =
                                    if let Some(ref album_artist) = album_details.artist {
                                        album_artist
                                            .name
                                            .as_ref()
                                            .unwrap_or(&"Unknown Artist".to_string())
                                            .clone()
                                    } else {
                                        "Unknown Artist".to_string()
                                    };

                                // Get album title
                                let album_title = album_details
                                    .title
                                    .as_ref()
                                    .unwrap_or(&"Unknown Album".to_string())
                                    .clone();

                                // Get track number and title
                                let track_number = track_details.track_number.unwrap_or(0);
                                let track_title = track_details
                                    .title
                                    .as_ref()
                                    .unwrap_or(&format!("Track {}", track_id))
                                    .clone();

                                // Create the directory structure: [Albuminterpret]/[Album]
                                let album_artist_dir = sanitize_filename(&album_artist_name);
                                let album_title_dir = sanitize_filename(&album_title);
                                let album_dir =
                                    format!("downloads/{}/{}", album_artist_dir, album_title_dir);

                                // Create filename following MusicBrainz Picard style: [Titelnr.]. [Titel]
                                let track_filename =
                                    format!("{:02}. {}", track_number, track_title);
                                let sanitized_filename = sanitize_filename(&track_filename);
                                let filename =
                                    format!("{}/{}.{}", album_dir, sanitized_filename, extension);

                                println!();
                                println!("Downloading track...");
                                println!();
                                match service
                                    .download_track(
                                        &track_id.to_string(),
                                        &quality,
                                        &filename,
                                        &config,
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        println!();
                                        println!("Track downloaded successfully!");
                                    }
                                    Err(e) => {
                                        println!();
                                        println!("Failed to download track: {}", e)
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("Search failed: {}", e),
            }
        }
    }
}

/// Presents an interactive quality selection menu to the user and returns the selected quality format ID.
///
/// The function displays available quality options and prompts the user to select one.
/// It handles user input validation and provides a default option if the input is invalid.
///
/// # Returns
///
/// * `Ok(String)` - The selected quality format ID as a string
///   - "5": MP3 320 kbps
///   - "6": FLAC Lossless
///   - "7": FLAC Hi-Res 24 bit <= 96kHz
///   - "27": FLAC Hi-Res 24 bit >96 kHz & <= 192 kHz
/// * `Err(QobuzApiError)` - If there's an error reading user input
///
/// # Example
///
/// ```text
/// Choose a quality:
/// 1) MP3 320 (format_id: 5)
/// 2) FLAC Lossless (format_id: 6)
/// 3) FLAC Hi-Res 24 bit <= 96kHz (format_id: 7)
/// 4) FLAC Hi-Res 24 bit >96 kHz & <= 192 kHz (format_id: 27)
/// ```
fn choose_quality() -> Result<String, QobuzApiError> {
    println!("\nChoose a quality:");
    println!("1) MP3 320 (format_id: 5)");
    println!("2) FLAC Lossless (format_id: 6)");
    println!("3) FLAC Hi-Res 24 bit <= 96kHz (format_id: 7)");
    println!("4) FLAC Hi-Res 24 bit >96 kHz & <= 192 kHz (format_id: 27)");

    let mut quality_choice = String::new();
    stdin()
        .read_line(&mut quality_choice)
        .expect("Failed to read line");
    let quality_choice = quality_choice.trim();

    match quality_choice {
        "1" => Ok("5".to_string()),
        "2" => Ok("6".to_string()),
        "3" => Ok("7".to_string()),
        "4" => Ok("27".to_string()),
        _ => {
            println!("Invalid choice, defaulting to FLAC Lossless.");
            Ok("6".to_string())
        }
    }
}

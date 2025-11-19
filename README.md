# Qobuz API Rust Client

An unofficial Rust client library for the Qobuz music streaming API, migrated from the C# implementation by [`DJDoubleD`](https://github.com/DJDoubleD). This library aims to provide comprehensive access to Qobuz's features, including authentication, content retrieval (albums, artists, tracks, playlists), search functionality, user favorites management, streaming URL generation, and advanced features like web player credential extraction and metadata embedding.

## Overview

This project is a migration of an existing C# Qobuz API library to Rust. The original C# project is written in .NET Framework 4.8.1, which is not compatible with Linux, making this Rust implementation particularly valuable for cross-platform use. The goal is to leverage Rust's strengths in memory safety, performance, and concurrency while maintaining the full functionality of the original C# version.

## Features

*   **Authentication:**
    *   Login with email, username, or authentication token.
    *   Password reset functionality.
*   **Content Retrieval:**
    *   Fetch detailed information for albums, artists, tracks, and playlists.
    *   Retrieve artist release lists.
*   **Search Functionality:**
    *   Search across the entire Qobuz catalog.
    *   Dedicated search for albums, artists, tracks, playlists, and articles.
*   **User Management:**
    *   Manage user favorites (add, delete, retrieve).
    *   (Future: User profile and subscription information retrieval).
*   **Streaming & Downloads:**
    *   Generate track file URLs for streaming.
    *   Download individual tracks and entire albums.
*   **Automatic Metadata Embedding:** 
    *   Downloads include embedding comprehensive metadata (artist, album, track details) into the audio files.
*   **Web Player Integration:** 
    *   Dynamically extract application ID and secret from the Qobuz Web Player's JavaScript bundle for easy initialization.
*   **Robust Error Handling:** 
    *   Custom error types for API responses, parsing issues, HTTP errors, and initialization problems.

## Migration Status

The migration from C# to Rust is largely complete in terms of core functionality. Most phases outlined in the `migration_plan.md` have been successfully implemented:

*   **Phase 1: Project Setup and Dependencies:** Completed.
*   **Phase 2: Core Data Structures:** Completed, with comprehensive models and `serde` for JSON handling.
*   **Phase 3: Utilities and Helpers:** Completed, including MD5 hashing, query string building, timestamp generation, and web player credential extraction logic.
*   **Phase 4: HTTP Client Implementation:** Completed, with robust asynchronous request handling using `reqwest` and `tokio`.
*   **Phase 5: Authentication System:** Completed, all login and password reset methods are implemented.
*   **Phase 6: Content Endpoints:** Completed, covering albums, artists, tracks, playlists, catalog, articles, and labels.
*   **Phase 7: User Functionality:** Completed, with full favorite management.
*   **Phase 8: Advanced Features:** Completed, including web player credential extraction and custom JSON deserialization where needed.

**Remaining areas for development:**

*   **Phase 9: Splitting Files:** Refactor [`embedder.rs`](src/metadata/embedder.rs) and [`utils.rs`](src/utils.rs)into smaller files to improve maintainability.
*   **Phase 10: Metadata:** Improve the metadata embedding to perfectly match the tagging of the C# implementation (refer to [`metadata_test/flac_metadata_report.md`](metadata_test/flac_metadata_report.md) and [`metadata_test/mp3_metadata_report.md`](metadata_test/mp3_metadata_report.md) for differences).
*   **Phase 11: Testing and Validation:** Comprehensive unit, integration, and functional tests are yet to be fully implemented.
*   **Phase 12: Documentation:** While code comments are present, comprehensive user-facing documentation and examples need to be expanded.
*   **Phase 13: Credential Refresh Optimization:** The credential refresh mechanism fetches new app credentials after each track download during album download when the stored credentials aren't valid, leading to unnecessary API calls. The refresh should be optimized to occur once per session or when needed for the entire album download process.

## Dependencies

The project utilizes the following key Rust crates:

*   [`base64`](https://crates.io/crates/base64): For Base64 encoding/decoding in credential extraction.
*   [`lofty`](https://crates.io/crates/lofty): For reading and writing audio metadata (used in track/album downloads).
*   [`md5`](https://crates.io/crates/md5): For MD5 hashing used in API request signing.
*   [`regex`](https://crates.io/crates/regex): For parsing web player JavaScript bundles.
*   [`reqwest`](https://crates.io/crates/reqwest): Asynchronous HTTP client.
*   [`serde`](https://crates.io/crates/serde) & [`serde_json`](https://crates.io/crates/serde_json): For efficient JSON serialization and deserialization.
*   [`thiserror`](https://crates.io/crates/thiserror): For ergonomic custom error types.
*   [`tokio`](https://crates.io/crates/tokio): Asynchronous runtime.
*   [`url`](https://crates.io/crates/url): For URL parsing utilities.

## Usage

### Instantiation and Application Authentication

The `QobuzApiService` can be instantiated in two ways:

1.  **Dynamic Credential Fetching (Recommended):** The service can attempt to fetch `app_id` and `app_secret` directly from the Qobuz Web Player.

    ```rust
    use std::error::Error;

    use qobuz_api_rust::QobuzApiService;

    #[main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let mut service = QobuzApiService::new().await?;
        println!("Qobuz API service initialized with app ID: {}", service.app_id);
        Ok(())
    }
    ```

    _Disclaimer: Fetching credentials from the web player may break at any time due to updates to the Qobuz Web Player._

2.  **With Provided Credentials:** If you have your `app_id` and `app_secret`, you can provide them directly.

    ```rust
    use std::error::Error;

    use qobuz_api_rust::QobuzApiService;

    #[main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let mut service = QobuzApiService::with_credentials(
            Some("YOUR_APP_ID".to_string()),
            Some("YOUR_APP_SECRET".to_string()),
        ).await?;
        println!("Qobuz API service initialized with app ID: {}", service.app_id);
        Ok(())
    }
    ```

### User Authentication

The library provides flexible authentication options with automatic credential detection. You can authenticate using one of the following methods:

#### 1. Automatic Authentication with Environment Variables (Recommended)

Create a `.env` file in your project root with one of the following credential combinations:

**Token-based authentication (most common):**
```env
QOBUZ_USER_ID=your_user_id_here
QOBUZ_USER_AUTH_TOKEN=your_auth_token_here
```

**Email and password authentication:**
```env
QOBUZ_EMAIL=your_email@example.com
QOBUZ_PASSWORD=your_md5_hashed_password_here
```

**Username and password authentication:**
```env
QOBUZ_USERNAME=your_username_here
QOBUZ_PASSWORD=your_md5_hashed_password_here
```

Then use the automatic authentication method:

```rust
use qobuz_api_rust::QobuzApiService;

// Load environment variables and attempt authentication
let mut service = QobuzApiService::new().await?;

match service.authenticate_with_env().await {
    Ok(login_result) => {
        println!("Authentication successful!");
        println!("User ID: {:?}", login_result.user.and_then(|u| u.id));
    }
    Err(e) => {
        println!("Authentication failed: {}", e);
    }
}
```

**Important Authentication Behavior Notes:** The Qobuz API seems to validates the user authentication token independently of the user ID. This should mean that as long as your `QOBUZ_USER_AUTH_TOKEN` is valid, you may still be able to access protected resources (like downloading full tracks) even if the `QOBUZ_USER_ID` doesn't match the token or is incorrect. The token validity is the primary factor for API access, not the user ID itself.

I haven't been able to test the login with email/password or username/password, as I only have access to user ID and user authentication token.

#### 2. Manual Authentication Methods

You can also use the specific login methods directly:

```rust
use qobuz_api_rust::QobuzApiService;

let mut service = QobuzApiService::new().await?;

// Token-based authentication
let login_result = service.login_with_token("user_id", "auth_token").await?;

// Email and password authentication (password should be MD5 hashed)
let login_result = service.login("email@example.com", "md5_hashed_password").await?;

// Username and password authentication (password should be MD5 hashed)
let login_result = service.login("username", "md5_hashed_password").await?;

// Or use the automatic authenticate method with explicit parameters
let login_result = service.authenticate(
    Some("user_id"),           // Optional user ID for token auth
    Some("user_auth_token"),   // Optional auth token
    Some("email"),             // Optional email for email auth
    Some("password"),          // Optional password (MD5 hashed)
    Some("username")           // Optional username for username auth
).await?;
```

### Searching and Downloading

After successful authentication, you can search for and download content:

```rust
// Example: Search for albums
let search_query = "Miles Davis";
let search_results = service.search_albums(search_query, Some(10), None, None).await?;

if let Some(albums) = search_results.albums {
    for album in albums.items.unwrap_or_default() {
        println!("Album: {} by {}", album.title.unwrap_or_default(), album.artist.and_then(|a| a.name).unwrap_or_default());
    }
}

// Example: Search for tracks
let track_search_query = "Kendrick Lamar";
let track_search_results = service.search_tracks(track_search_query, Some(10), None, None).await?;

if let Some(tracks) = track_search_results.tracks {
    for (i, track) in tracks.items.unwrap_or_default().iter().enumerate() {
        println!("{}) {} - {}",
            i + 1,
            track.performer.as_ref().and_then(|a| a.name.as_deref()).unwrap_or("Unknown Artist"),
            track.title.as_deref().unwrap_or("No title")
        );
    }
}

// Example: Download a track (assuming you have a track ID and format ID)
// Track format IDs:
// 5 - MP3 320
// 6 - FLAC Lossless
// 7 - FLAC Hi-Res 24 bit <= 96kHz
// 27 - FLAC Hi-Res 24 bit >96 kHz & <= 192 kHz

let track_id = "40128300"; // Example track ID
let format_id = "6"; // FLAC Lossless
let download_path = "downloads/Artist/Album/01. TrackTitle.flac";

service.download_track(track_id, format_id, download_path).await?;
println!("Track downloaded to {}", download_path);

// Example: Download an entire album
let album_id = "12345"; // Example album ID
let quality = "6"; // FLAC Lossless
let album_path = "downloads/Artist/Album/";

service.download_album(album_id, quality, album_path).await?;
println!("Album downloaded to {}", album_path);
```

For more detailed usage, refer to the source code and the `src/main.rs` example.

## Acknowledgements

This project is inspired by and built upon the excellent work of [`DJDoubleD`](https://github.com/DJDoubleD). Special thanks to him and his projects:

*   **DJDoubleD/QobuzApiSharp:** The original C# Qobuz API library that served as the foundation and migration source for this Rust client.
    *   [https://github.com/DJDoubleD/QobuzApiSharp](https://github.com/DJDoubleD/QobuzApiSharp)
*   **DJDoubleD/QobuzDownloaderX-MOD:** For insights into Qobuz API interactions and related tooling.
    *   [https://github.com/DJDoubleD/QobuzDownloaderX-MOD](https://github.com/DJDoubleD/QobuzDownloaderX-MOD)

## License

This project is licensed under the GNU General Public License v3.0 - see the [`LICENSE.txt`](LICENSE.txt) file for details.
use std::fs::create_dir_all;

use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self, ApiErrorResponse, IoError},
    models::{Album, SearchResult},
    utils::sanitize_filename,
};

impl QobuzApiService {
    /// Retrieves an album with the specified ID.
    ///
    /// This method fetches detailed information about a specific album from the Qobuz API,
    /// including metadata, track listing, and other album-related information.
    ///
    /// # Arguments
    ///
    /// * `album_id` - The unique identifier of the album to retrieve
    /// * `with_auth` - Optional boolean to execute request with or without user authentication token.
    ///   When `None`, defaults to `false` (no authentication).
    /// * `extra` - Optional string specifying additional album information to include in the response,
    ///   such as "items", "tracks", "release_tags", etc.
    /// * `limit` - Optional integer specifying the maximum number of tracks to include in the response.
    ///   When `None`, defaults to 1200.
    /// * `offset` - Optional integer specifying the offset of the first track to include in the response.
    ///   When `None`, defaults to 0.
    ///
    /// # Returns
    ///
    /// * `Ok(Album)` - Contains the complete album information if the request is successful
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues, invalid parameters,
    ///   or other API-related errors
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// let album = service.get_album("12345", None, Some("tracks"), Some(10), None).await?;
    /// println!("Album title: {}", album.title.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_album(
        &self,
        album_id: &str,
        with_auth: Option<bool>,
        extra: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Album, QobuzApiError> {
        let mut params = vec![("album_id".to_string(), album_id.to_string())];

        if let Some(extra_val) = extra {
            params.push(("extra".to_string(), extra_val.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(1200).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/album/get", &params).await
    }

    /// Searches for albums using the specified query.
    ///
    /// This method allows searching for albums based on a text query, with optional pagination
    /// parameters to control the number of results returned.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string (e.g., album title, artist name)
    /// * `limit` - Optional integer specifying the maximum number of results to return.
    ///   When `None`, defaults to 50.
    /// * `offset` - Optional integer specifying the offset of the first result to return.
    ///   When `None`, defaults to 0.
    /// * `with_auth` - Optional boolean to execute search with or without user authentication token.
    ///   When `None`, defaults to `false` (no authentication).
    ///
    /// # Returns
    ///
    /// * `Ok(SearchResult)` - Contains the search results with albums matching the query
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues, invalid parameters,
    ///   or other API-related errors
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// let results = service.search_albums("radiohead", Some(10), None, None).await?;
    /// if let Some(albums) = results.albums {
    ///     println!("Found {} albums", albums.total.unwrap_or(0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_albums(
        &self,
        query: &str,
        limit: Option<i32>,
        offset: Option<i32>,
        with_auth: Option<bool>,
    ) -> Result<SearchResult, QobuzApiError> {
        let params = vec![
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), limit.unwrap_or(50).to_string()),
            ("offset".to_string(), offset.unwrap_or(0).to_string()),
        ];

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/album/search", &params).await
    }

    /// Downloads an entire album to the specified path.
    ///
    /// This method downloads all tracks of an album to a specified directory, with options for
    /// different audio quality formats. The method handles track-by-track downloads and includes
    /// automatic credential refresh if signature errors occur during the download process.
    ///
    /// # Arguments
    ///
    /// * `album_id` - The unique identifier of the album to download
    /// * `format_id` - The format ID specifying audio quality:
    ///   - "5": MP3 320 kbps
    ///   - "6": FLAC Lossless (16-bit/44.1kHz)
    ///   - "7": FLAC Hi-Res (24-bit/96kHz)
    ///   - "27": FLAC Hi-Res (24-bit/192kHz)
    /// * `path` - The directory path where the album should be saved. The directory will be created
    ///   if it doesn't exist. The path should already include artist/album folder structure.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If all tracks in the album are downloaded successfully
    /// * `Err(QobuzApiError)` - If the API request fails, download fails for any track, or other
    ///   errors occur during the process
    ///
    /// # Note
    ///
    /// This method includes automatic retry with credential refresh if signature errors occur.
    /// Each track is downloaded with progress reporting and metadata embedding.
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// service.download_album("12345", "6", "./downloads/Artist/Album Title").await?;
    /// println!("Album downloaded successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_album(
        &self,
        album_id: &str,
        format_id: &str,
        path: &str,
    ) -> Result<(), QobuzApiError> {
        let album = self
            .get_album(album_id, None, Some("track_ids"), None, None)
            .await?;

        if let Some(track_ids) = album.track_ids {
            let total_tracks = track_ids.len();
            println!();
            println!("Album contains {} tracks", total_tracks);
            println!();

            // Create the directory structure as provided in path parameter
            let album_dir = path;
            create_dir_all(album_dir).map_err(IoError)?;

            for (index, track_id) in track_ids.iter().enumerate() {
                let track = self.get_track(&track_id.to_string(), None).await?;
                let file_extension = match format_id {
                    "5" => "mp3",
                    "6" | "7" | "27" => "flac",
                    _ => "flac", // default to flac
                };

                // Get track number and title for the filename
                let track_number = track.track_number.unwrap_or(0);
                let track_title = track
                    .title
                    .as_ref()
                    .unwrap_or(&format!("Track {}", track_id))
                    .clone();

                // Create filename following MusicBrainz Picard style: [Titelnr.]. [Titel]
                let track_filename = format!("{:02}. {}", track_number, track_title);
                let sanitized_filename = sanitize_filename(&track_filename);
                let track_path = format!("{}/{}.{}", album_dir, sanitized_filename, file_extension);

                println!(
                    "Downloading track {}/{}: {} - {}",
                    index + 1,
                    total_tracks,
                    track_number,
                    track_title
                );

                // Attempt to download the track, with credential refresh on signature errors
                match self
                    .download_track(&track_id.to_string(), format_id, &track_path)
                    .await
                {
                    Ok(()) => {
                        // Success, continue to next track
                    }

                    Err(ApiErrorResponse { message, .. })
                        if message.contains("Invalid Request Signature parameter") =>
                    {
                        eprintln!(
                            "Invalid signature detected during album download, attempting to refresh app credentials..."
                        );

                        // Refresh credentials and retry the track download
                        match self.refresh_app_credentials().await {
                            Ok(new_service) => {
                                // Use the new service instance to download the track
                                match new_service
                                    .download_track(&track_id.to_string(), format_id, &track_path)
                                    .await
                                {
                                    Ok(()) => {
                                        // Successfully downloaded with new credentials
                                    }

                                    Err(e) => {
                                        // If it still fails, return the error
                                        return Err(e);
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("Failed to refresh credentials: {}", e);
                                return Err(ApiErrorResponse {
                                    code: 400.to_string(),
                                    message,
                                    status: "error".to_string(),
                                });
                            }
                        }
                    }

                    Err(e) => {
                        // Any other error, return immediately
                        return Err(e);
                    }
                }
            }

            println!();
            println!(
                "Album download completed: {}/{} tracks downloaded",
                total_tracks, total_tracks
            );
            println!();
        }

        Ok(())
    }
}

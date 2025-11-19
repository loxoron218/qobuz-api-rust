use std::{
    fs::{File, create_dir_all},
    io::{BufWriter, Write, stdout},
    path::Path,
};

use {reqwest::header::CONTENT_LENGTH, tokio_stream::StreamExt};

use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{
        self, ApiErrorResponse, DownloadError, HttpError, MetadataError, ResourceNotFoundError,
    },
    metadata::embedder::embed_metadata_in_file,
    models::{FileUrl, SearchResult, Track},
    utils::{get_current_timestamp, get_md5_hash},
};

impl QobuzApiService {
    /// Generates the signature for the getFileUrl endpoint.
    ///
    /// This internal function creates a signature required to access track file URLs from the Qobuz API.
    /// The signature is created by concatenating specific parameters with the app secret and hashing
    /// the result using MD5.
    ///
    /// # Arguments
    /// * `format_id` - The format ID for the desired audio quality
    /// * `track_id` - The unique identifier of the track
    /// * `timestamp` - The current timestamp to ensure request freshness
    ///
    /// # Returns
    /// A hexadecimal string representing the MD5 hash of the signature data
    fn generate_get_file_url_signature(
        &self,
        format_id: &str,
        track_id: &str,
        timestamp: &str,
    ) -> String {
        let data_to_sign = format!(
            "trackgetFileUrlformat_id{}intentstreamtrack_id{}{}{}",
            format_id, track_id, timestamp, self.app_secret
        );

        get_md5_hash(&data_to_sign)
    }

    /// Retrieves detailed information about a specific track by its ID.
    ///
    /// This function fetches comprehensive metadata about a track including its title,
    /// duration, album information, performer details, and audio specifications.
    /// The request can optionally be made with authentication for access to additional
    /// content or features.
    ///
    /// # Arguments
    /// * `track_id` - The unique identifier of the track to retrieve
    /// * `with_auth` - Whether to execute the request with the user authentication token
    ///   (optional, defaults to false). When `true`, the request includes
    ///   the user's authentication token if available.
    ///
    /// # Returns
    /// * `Ok(Track)` - The complete track information if the request succeeds
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues,
    ///   invalid parameters, or API errors
    ///
    /// # Example
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    /// let track = service.get_track("12345", None).await?;
    /// println!("Track title: {:?}", track.title);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_track(
        &self,
        track_id: &str,
        with_auth: Option<bool>,
    ) -> Result<Track, QobuzApiError> {
        let params = vec![("track_id".to_string(), track_id.to_string())];

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/track/get", &params).await
    }

    /// Retrieves the download URL for a track in a specific audio format.
    ///
    /// This function obtains a direct URL to download the audio file for a track in the specified format.
    /// The Qobuz API requires a signature for this endpoint, which is automatically generated and
    /// validated. If the signature is invalid (which may happen with expired app credentials),
    /// the function will attempt to refresh the credentials and retry the request.
    ///
    /// # Arguments
    /// * `track_id` - The unique identifier of the track
    /// * `format_id` - The format ID specifying the audio quality:
    ///   - `5` for MP3 320 kbps
    ///   - `6` for FLAC Lossless (16-bit/44.1kHz)
    ///   - `7` for FLAC Hi-Res (24-bit, ≤96kHz)
    ///   - `27` for FLAC Hi-Res (24-bit, >96kHz & ≤192kHz)
    ///
    /// # Returns
    /// * `Ok(FileUrl)` - Contains the download URL and metadata about the audio file if successful
    /// * `Err(QobuzApiError)` - If the API request fails, credentials are invalid, or the track/format is unavailable
    ///
    /// # Note
    /// This endpoint requires authentication and may automatically refresh app credentials if needed.
    ///
    /// # Example
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    /// let file_url = service.get_track_file_url("12345", "6").await?;
    /// if let Some(url) = file_url.url {
    ///     println!("Download URL: {}", url);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_track_file_url(
        &self,
        track_id: &str,
        format_id: &str,
    ) -> Result<FileUrl, QobuzApiError> {
        let timestamp = get_current_timestamp();
        let signature = self.generate_get_file_url_signature(format_id, track_id, &timestamp);

        let params = vec![
            ("track_id".to_string(), track_id.to_string()),
            ("format_id".to_string(), format_id.to_string()),
            ("intent".to_string(), "stream".to_string()),
            ("request_ts".to_string(), timestamp),
            ("request_sig".to_string(), signature),
        ];

        // This endpoint requires authentication
        match self.get("/track/getFileUrl", &params).await {
            Ok(result) => Ok(result),
            Err(ApiErrorResponse {
                code,
                message,
                status,
            }) => {
                // Check if this is the signature error that indicates invalid app credentials
                if message.contains("Invalid Request Signature parameter") {
                    eprintln!(
                        "Invalid signature detected, attempting to refresh app credentials..."
                    );

                    // Fetch new credentials
                    match self.refresh_app_credentials().await {
                        Ok(new_service) => {
                            // Retry the request with new credentials
                            let new_timestamp = get_current_timestamp();
                            let new_signature = new_service.generate_get_file_url_signature(
                                format_id,
                                track_id,
                                &new_timestamp,
                            );

                            let new_params = vec![
                                ("track_id".to_string(), track_id.to_string()),
                                ("format_id".to_string(), format_id.to_string()),
                                ("intent".to_string(), "stream".to_string()),
                                ("request_ts".to_string(), new_timestamp),
                                ("request_sig".to_string(), new_signature),
                            ];

                            new_service.get("/track/getFileUrl", &new_params).await
                        }

                        Err(e) => {
                            eprintln!("Failed to refresh credentials: {}", e);
                            Err(ApiErrorResponse {
                                code,
                                message,
                                status,
                            })
                        }
                    }
                } else {
                    // Return the original error if it's not a signature error
                    Err(ApiErrorResponse {
                        code,
                        message,
                        status,
                    })
                }
            }

            Err(e) => Err(e),
        }
    }

    /// Searches for tracks based on a text query with optional pagination and authentication.
    ///
    /// This function performs a text-based search across Qobuz's track catalog, allowing users
    /// to find tracks by title, artist, album, or other metadata. The search can be customized
    /// with pagination parameters and optional authentication for enhanced results.
    ///
    /// # Arguments
    /// * `query` - The search term to look for in track metadata (title, artist, etc.)
    /// * `limit` - The maximum number of results to return (optional, defaults to 50, maximum 500)
    /// * `offset` - The offset of the first result to return (optional, defaults to 0)
    ///   Use this for pagination to retrieve subsequent result sets
    /// * `with_auth` - Whether to execute the search with the user authentication token
    ///   (optional, defaults to false). When `true`, the request includes
    ///   the user's authentication token if available, potentially returning
    ///   personalized or higher-quality results.
    ///
    /// # Returns
    /// * `Ok(SearchResult)` - Contains the search results with track information and metadata
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues, invalid
    ///   parameters, or API errors
    ///
    /// # Example
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    /// let results = service.search_tracks("Bohemian Rhapsody", Some(10), None, None).await?;
    /// if let Some(tracks) = results.tracks {
    ///     println!("Found {} tracks", tracks.total.unwrap_or(0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_tracks(
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

        self.get("/track/search", &params).await
    }

    /// Downloads a track to the specified file path with embedded metadata.
    ///
    /// This function downloads a track from Qobuz in the specified audio format and saves it
    /// to the provided file path. After downloading, it automatically embeds comprehensive
    /// metadata (title, artist, album, cover art, etc.) into the audio file using the
    /// metadata embedding functionality.
    ///
    /// # Arguments
    /// * `track_id` - The unique identifier of the track to download
    /// * `format_id` - The format ID specifying the audio quality:
    ///   - `5` for MP3 320 kbps
    ///   - `6` for FLAC Lossless (16-bit/44.1kHz)
    ///   - `7` for FLAC Hi-Res (24-bit, ≤96kHz)
    ///   - `27` for FLAC Hi-Res (24-bit, >96kHz & ≤192kHz)
    /// * `path` - The file system path where the track should be saved
    ///
    /// # Returns
    /// * `Ok(())` - If the track was successfully downloaded and metadata was embedded
    /// * `Err(QobuzApiError)` - If the API request fails, download fails, directory creation
    ///   fails, or metadata embedding fails
    ///
    /// # Note
    /// This function displays download progress in the console. The function will attempt
    /// to create the target directory if it doesn't exist.
    ///
    /// # Example
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    /// service.download_track("12345", "6", "./downloads/track.flac").await?;
    /// println!("Track downloaded successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_track(
        &self,
        track_id: &str,
        format_id: &str,
        path: &str,
    ) -> Result<(), QobuzApiError> {
        match self.get_track_file_url(track_id, format_id).await {
            Ok(file_url) => {
                if let Some(url) = file_url.url {
                    let response =
                        self.client
                            .get(&url)
                            .send()
                            .await
                            .map_err(|e| DownloadError {
                                message: format!("Failed to initiate download: {}", e),
                            })?;

                    // Check if the response is successful
                    if !response.status().is_success() {
                        return Err(HttpError(response.error_for_status().unwrap_err()));
                    }

                    // Create the directory if it doesn't exist
                    if let Some(parent) = Path::new(path).parent() {
                        create_dir_all(parent).map_err(|e| DownloadError {
                            message: format!("Failed to create directory: {}", e),
                        })?;
                    }

                    // Get the total content length if available
                    let content_length = response
                        .headers()
                        .get(CONTENT_LENGTH)
                        .and_then(|len| len.to_str().ok())
                        .and_then(|len| len.parse::<u64>().ok());

                    // Create a file to write the response to
                    let mut dest =
                        BufWriter::new(File::create(path).map_err(|e| DownloadError {
                            message: format!("Failed to create file: {}", e),
                        })?);

                    // Get the response body as bytes stream
                    let mut stream = response.bytes_stream();

                    let mut downloaded: u64 = 0;

                    while let Some(chunk_result) = stream.next().await {
                        let chunk = chunk_result.map_err(|e| DownloadError {
                            message: format!("Failed to read chunk from response stream: {}", e),
                        })?;
                        dest.write_all(&chunk).map_err(|e| DownloadError {
                            message: format!("Failed to write chunk to file: {}", e),
                        })?;
                        downloaded += chunk.len() as u64;

                        // Print progress if we know the total size
                        if let Some(total) = content_length {
                            print!(
                                "\rProgress: {}/{} bytes ({:.2}%)",
                                downloaded,
                                total,
                                (downloaded as f64 / total as f64) * 100.0
                            );
                        } else {
                            print!("\rDownloaded: {} bytes", downloaded);
                        }
                        stdout().flush().map_err(|e| DownloadError {
                            message: format!("Failed to flush stdout: {}", e),
                        })?;
                    }

                    // Add a new line after progress display
                    println!();

                    // Flush the writer to ensure all data is written
                    dest.flush().map_err(|e| DownloadError {
                        message: format!("Failed to flush file writer: {}", e),
                    })?;

                    // After downloading, fetch track, album, and artist details to embed metadata
                    let track =
                        self.get_track(track_id, None)
                            .await
                            .map_err(|e| DownloadError {
                                message: format!("Failed to get track details for metadata: {}", e),
                            })?;
                    let album = if let Some(ref track_album) = track.album {
                        track_album.as_ref().clone()
                    } else {
                        return Err(ResourceNotFoundError {
                            resource_type: "album".to_string(),
                            resource_id: track_id.to_string(),
                        });
                    };

                    let artist = if let Some(ref track_artist) = track.performer {
                        track_artist.as_ref().clone()
                    } else if let Some(album_artist) = &album.artist {
                        album_artist.as_ref().clone()
                    } else {
                        return Err(ResourceNotFoundError {
                            resource_type: "artist".to_string(),
                            resource_id: track_id.to_string(),
                        });
                    };

                    // Embed metadata in the downloaded file
                    println!("Embedding metadata in {}", path);
                    embed_metadata_in_file(path, &track, &album, &artist)
                        .await
                        .map_err(|e| MetadataError {
                            source: Box::new(e),
                        })?;

                    Ok(())
                } else {
                    Err(DownloadError {
                        message: "No download URL found for the track".to_string(),
                    })
                }
            }

            Err(e) => Err(e),
        }
    }
}

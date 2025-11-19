use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self},
    models::{Playlist, SearchResult},
};

impl QobuzApiService {
    /// Retrieves a specific playlist by its ID from the Qobuz API.
    ///
    /// This method fetches detailed information about a single playlist, including its metadata,
    /// tracks, and other associated information. The playlist ID uniquely identifies the playlist
    /// in the Qobuz system.
    ///
    /// # Arguments
    ///
    /// * `playlist_id` - The unique identifier of the playlist to retrieve
    /// * `with_auth` - Optional boolean to execute request with user authentication token.
    ///   If `None` or `Some(false)`, the request is made without authentication.
    ///   If `Some(true)`, the request uses the stored authentication token if available.
    /// * `extra` - Optional string specifying additional information to include in the response.
    ///   This can be used to request additional metadata or related content.
    /// * `limit` - Optional integer specifying the maximum number of extra results to return.
    ///   Defaults to 25 if not specified.
    /// * `offset` - Optional integer specifying the offset of the first extra result to return.
    ///   Defaults to 0 if not specified.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    ///
    /// // Get a playlist by ID without additional parameters
    /// let playlist = service.get_playlist("12345", None, None, None, None).await?;
    /// println!("Playlist name: {:?}", playlist.name);
    ///
    /// // Get a playlist with authentication and extra data
    /// let playlist = service.get_playlist(
    ///     "12345",
    ///     Some(true),
    ///     Some("tracks,owner"),
    ///     Some(50),
    ///     Some(0)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(Playlist)` - Successfully retrieved playlist with all available information
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues, invalid parameters,
    ///   authentication problems, or if the playlist doesn't exist
    pub async fn get_playlist(
        &self,
        playlist_id: &str,
        with_auth: Option<bool>,
        extra: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Playlist, QobuzApiError> {
        let mut params = vec![("playlist_id".to_string(), playlist_id.to_string())];

        if let Some(extra_val) = extra {
            params.push(("extra".to_string(), extra_val.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(25).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/playlist/get", &params).await
    }

    /// Searches for playlists using a text query through the Qobuz API.
    ///
    /// This method allows you to search for playlists based on a text query, which can match
    /// playlist names, descriptions, or other relevant metadata. The results are returned as
    /// a paginated list of playlists that match the search criteria.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string to match against playlist names, descriptions, etc.
    /// * `limit` - Optional integer specifying the maximum number of results to return.
    ///   Defaults to 50 if not specified. Maximum values may be enforced by the API.
    /// * `offset` - Optional integer specifying the offset of the first result to return,
    ///   used for pagination. Defaults to 0 if not specified.
    /// * `with_auth` - Optional boolean to execute search with user authentication token.
    ///   If `None` or `Some(false)`, the search is made without authentication.
    ///   If `Some(true)`, the search uses the stored authentication token if available.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let service = QobuzApiService::new().await?;
    ///
    /// // Search for playlists containing "chill" without authentication
    /// let results = service.search_playlists("chill", None, None, None).await?;
    /// if let Some(playlists) = &results.playlists {
    ///     println!("Found {} playlists", playlists.total.unwrap_or(0));
    /// }
    ///
    /// // Search with pagination and authentication
    /// let results = service.search_playlists("jazz", Some(20), Some(40), Some(true)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(SearchResult)` - Successfully retrieved search results containing matching playlists
    ///   and metadata about the search (total count, pagination info, etc.)
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues, invalid parameters,
    ///   authentication problems, or other API-related errors
    pub async fn search_playlists(
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

        self.get("/playlist/search", &params).await
    }
}

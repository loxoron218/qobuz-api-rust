use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self},
    models::SearchResult,
};

/// Provides functionality for interacting with the Qobuz catalog API.
///
/// This module contains methods for searching the Qobuz music catalog, allowing
/// users to find albums, artists, tracks, and other content based on various criteria.
impl QobuzApiService {
    /// Searches the Qobuz catalog for content matching the specified query.
    ///
    /// This method allows you to search across the entire Qobuz catalog or filter
    /// results by a specific content type. The search is performed using the Qobuz API
    /// and returns a [`SearchResult`] containing the matching items.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query string. This can be an artist name, album title,
    ///   track title, or any other text to search for in the catalog.
    /// * `limit` - The maximum number of results to return. Defaults to 50 if not specified.
    ///   The API may return fewer results than requested depending on the search criteria.
    /// * `offset` - The offset of the first result to return, used for pagination.
    ///   Defaults to 0 if not specified, which returns results starting from the first match.
    /// * `type_param` - Optional parameter to limit results to a specific content type.
    ///   Valid values include "albums", "artists", "tracks", "playlists", "labels", etc.
    ///   If not specified, results from all content types will be returned.
    /// * `with_auth` - Whether to execute the search with the user's authentication token.
    ///   This parameter is currently unused in the implementation but is reserved for
    ///   future use to enable authenticated searches that may return personalized results.
    ///
    /// # Examples
    ///
    /// ```
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// let api = QobuzApiService::new().await?;
    ///
    /// // Search for "Billie Eilish" across all content types
    /// let results = api.search_catalog("Billie Eilish", None, None, None, None).await?;
    ///
    /// // Search for only albums by "Billie Eilish", limited to 10 results
    /// let album_results = api.search_catalog("Billie Eilish", Some(10), None, Some("albums"), None).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// * `Ok(SearchResult)` - Contains the search results with matching albums, artists,
    ///   tracks, and other content based on the query. The structure includes separate
    ///   sections for each content type that had matching results.
    /// * `Err(QobuzApiError)` - If the API request fails due to network issues,
    ///   authentication problems, invalid parameters, or other API-related errors.
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    ///
    /// * Network connectivity issues preventing the API request
    /// * Invalid API credentials (app ID or app secret)
    /// * Invalid parameters passed to the API
    /// * API rate limiting
    /// * Server-side errors from the Qobuz API
    /// * JSON parsing errors when processing the API response
    ///
    /// # API Endpoint
    ///
    /// This method calls the `/catalog/search` endpoint of the Qobuz API.
    pub async fn search_catalog(
        &self,
        query: &str,
        limit: Option<i32>,
        offset: Option<i32>,
        type_param: Option<&str>,
        with_auth: Option<bool>,
    ) -> Result<SearchResult, QobuzApiError> {
        let mut params = vec![
            ("query".to_string(), query.to_string()),
            ("limit".to_string(), limit.unwrap_or(50).to_string()),
            ("offset".to_string(), offset.unwrap_or(0).to_string()),
        ];

        if let Some(type_val) = type_param {
            params.push(("type".to_string(), type_val.to_string()));
        }

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/catalog/search", &params).await
    }
}

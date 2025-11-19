use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self},
    models::{Artist, ReleasesList, SearchResult},
};

/// Parameters for the artist release list API.
///
/// This struct contains all the configurable options for retrieving an artist's releases
/// from the Qobuz API. It allows for filtering by release type, sorting options, pagination,
/// and authentication preferences.
///
/// # Example
///
/// ```
/// use qobuz_api_rust::api::content::artists::ArtistReleaseListParams;
///
/// let params = ArtistReleaseListParams {
///     with_auth: Some(true),
///     release_type: Some("album".to_string()),
///     sort: Some("release_date".to_string()),
///     order: Some("desc".to_string()),
///     track_size: Some(5),
///     limit: Some(20),
///     offset: Some(0),
/// };
/// ```
#[derive(Default, Debug)]
pub struct ArtistReleaseListParams {
    /// Whether to execute the request with authentication (user_auth_token)
    ///
    /// When `Some(true)`, the request will be made with the authenticated user's token.
    /// When `Some(false)` or `None`, the request will be made without authentication.
    pub with_auth: Option<bool>,
    /// The type of releases to include in the results
    ///
    /// Possible values include "album", "single", "compilation", etc.
    /// If `None`, all release types will be included.
    pub release_type: Option<String>,
    /// The sorting criterion for releases
    ///
    /// Common values include "release_date", "title", "popularity", etc.
    /// If `None`, the default sorting (usually by release date) will be used.
    pub sort: Option<String>,
    /// The order direction for sorting
    ///
    /// Use "asc" for ascending or "desc" for descending order.
    /// If `None`, the default order (usually "desc") will be used.
    pub order: Option<String>,
    /// The maximum number of tracks to include in each release
    ///
    /// This limits how many tracks are returned for each album/single in the results.
    /// If `None`, defaults to 10 tracks per release.
    pub track_size: Option<i32>,
    /// The maximum number of releases to return
    ///
    /// Use this to limit the number of releases in the response for pagination.
    /// If `None`, defaults to 50 releases.
    pub limit: Option<i32>,
    /// The offset for pagination
    ///
    /// Use this to skip a certain number of releases from the beginning of the results.
    /// If `None`, defaults to 0 (no offset).
    pub offset: Option<i32>,
}

impl QobuzApiService {
    /// Retrieves detailed information about an artist using their unique ID.
    ///
    /// This method fetches comprehensive information about a specific artist from the Qobuz API.
    /// The response includes basic artist details as well as potentially extended information
    /// based on the `extra` parameter.
    ///
    /// # Arguments
    ///
    /// * `artist_id` - The unique identifier of the artist to retrieve
    /// * `with_auth` - Whether to execute the request with user authentication (optional, defaults to false)
    /// * `extra` - Additional information to include in the response, such as albums, playlists, etc. (optional)
    /// * `sort` - How to sort the requested extra information (optional)
    /// * `limit` - Maximum number of extra results to return (optional, defaults to 50)
    /// * `offset` - Offset of the first extra result to return (optional, defaults to 0)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Artist)` containing the artist information if successful, or `Err(QobuzApiError)`
    /// if the API request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// let artist = service.get_artist("12345", None, Some("albums"), None, None, None).await?;
    /// println!("Artist name: {:?}", artist.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_artist(
        &self,
        artist_id: &str,
        with_auth: Option<bool>,
        extra: Option<&str>,
        sort: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Artist, QobuzApiError> {
        let mut params = vec![("artist_id".to_string(), artist_id.to_string())];

        if let Some(extra_val) = extra {
            params.push(("extra".to_string(), extra_val.to_string()));
        }

        if let Some(sort_val) = sort {
            params.push(("sort".to_string(), sort_val.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(50).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/artist/get", &params).await
    }

    /// Retrieves a list of releases for the specified artist.
    ///
    /// This method fetches all releases (albums, singles, etc.) associated with a specific artist.
    /// The results can be filtered and sorted based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `artist_id` - The unique identifier of the artist whose releases to fetch
    /// * `params` - Configuration parameters for the request, including filtering, sorting, and pagination options
    ///
    /// # Returns
    ///
    /// Returns `Ok(ReleasesList)` containing the list of releases if successful, or `Err(QobuzApiError)`
    /// if the API request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # use qobuz_api_rust::api::content::artists::ArtistReleaseListParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// let params = ArtistReleaseListParams {
    ///     release_type: Some("album".to_string()),
    ///     sort: Some("release_date".to_string()),
    ///     order: Some("desc".to_string()),
    ///     ..Default::default()
    /// };
    /// let releases = service.get_release_list("12345", params).await?;
    /// println!("Found {} releases", releases.items.as_ref().map(|items| items.len()).unwrap_or(0));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_release_list(
        &self,
        artist_id: &str,
        params: ArtistReleaseListParams,
    ) -> Result<ReleasesList, QobuzApiError> {
        let mut query_params = vec![("artist_id".to_string(), artist_id.to_string())];

        if let Some(release_type_val) = params.release_type {
            query_params.push(("release_type".to_string(), release_type_val));
        }

        if let Some(sort_val) = params.sort {
            query_params.push(("sort".to_string(), sort_val));
        }

        if let Some(order_val) = params.order {
            query_params.push(("order".to_string(), order_val));
        }

        query_params.push((
            "track_size".to_string(),
            params.track_size.unwrap_or(10).to_string(),
        ));
        query_params.push(("limit".to_string(), params.limit.unwrap_or(50).to_string()));
        query_params.push(("offset".to_string(), params.offset.unwrap_or(0).to_string()));

        let _use_auth = params.with_auth.unwrap_or(false);

        self.get("/artist/getReleasesList", &query_params).await
    }

    /// Searches for artists matching the specified query.
    ///
    /// This method performs a text-based search across artist names and other metadata
    /// to find artists that match the provided query string.
    ///
    /// # Arguments
    ///
    /// * `query` - The search term to look for in artist names and metadata
    /// * `limit` - Maximum number of results to return (optional, defaults to 50)
    /// * `offset` - Offset for pagination (optional, defaults to 0)
    /// * `with_auth` - Whether to execute the search with user authentication (optional, defaults to false)
    ///
    /// # Returns
    ///
    /// Returns `Ok(SearchResult)` containing the search results if successful, or `Err(QobuzApiError)`
    /// if the API request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # use qobuz_api_rust::QobuzApiService;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let service = QobuzApiService::new().await?;
    /// let results = service.search_artists("Beatles", Some(10), None, None).await?;
    /// if let Some(artists) = results.artists {
    ///     println!("Found {} artists", artists.items.as_ref().map(|items| items.len()).unwrap_or(0));
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_artists(
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

        self.get("/artist/search", &params).await
    }
}

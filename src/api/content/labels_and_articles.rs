use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self},
    models::{Label, SearchResult},
};

impl QobuzApiService {
    /// Searches for articles using the specified query.
    ///
    /// This method allows you to search for articles on the Qobuz platform using a text query.
    /// The results can be paginated and limited to a specific number of entries.
    ///
    /// ## Parameters
    ///
    /// - `query`: The search query string to match against article titles, descriptions, and content
    /// - `limit`: Optional maximum number of results to return (defaults to 50 if not specified)
    /// - `offset`: Optional offset for pagination (defaults to 0 if not specified)
    /// - `with_auth`: Optional flag to execute the search with user authentication (defaults to false)
    ///
    /// ## Returns
    ///
    /// - `Ok(SearchResult)`: Contains the search results including articles that match the query
    /// - `Err(QobuzApiError)`: If the API request fails due to network issues, authentication problems, or invalid parameters
    ///
    /// ## Examples
    ///
    /// Basic search for articles:
    ///
    /// ```rust
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// # let api = QobuzApiService::new().await?;
    /// let results = api.search_articles("classical music", None, None, None).await?;
    /// println!("Found {} articles", results.articles.as_ref().map(|a| a.total.unwrap_or(0)).unwrap_or(0));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Search with custom limit and offset for pagination:
    ///
    /// ```rust
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// # let api = QobuzApiService::new().await?;
    /// let results = api.search_articles("jazz", Some(10), Some(20), None).await?;
    /// // Gets 10 articles starting from the 20th result
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_articles(
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

        self.get("/article/search", &params).await
    }

    /// Gets Label with the specified label ID.
    ///
    /// This method retrieves detailed information about a specific music label using its unique ID.
    /// Additional related content can be included in the response based on the `extra` parameter.
    ///
    /// ## Parameters
    ///
    /// - `label_id`: The unique identifier of the label to retrieve
    /// - `with_auth`: Optional flag to execute the request with user authentication (defaults to false)
    /// - `extra`: Optional string specifying additional data to include (e.g., "albums", "news", "articles")
    /// - `limit`: Optional maximum number of extra results to return (defaults to 25 if not specified)
    /// - `offset`: Optional offset for pagination of extra results (defaults to 0 if not specified)
    ///
    /// ## Returns
    ///
    /// - `Ok(Label)`: Contains the detailed information about the requested label
    /// - `Err(QobuzApiError)`: If the API request fails due to network issues, authentication problems, or invalid parameters
    ///
    /// ## Examples
    ///
    /// Basic label retrieval:
    ///
    /// ```rust
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// # let api = QobuzApiService::new().await?;
    /// let label = api.get_label("12345", None, None, None, None).await?;
    /// println!("Label name: {:?}", label.name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Get label with additional albums:
    ///
    /// ```rust
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # async fn example() -> Result<(), QobuzApiError> {
    /// # let api = QobuzApiService::new().await?;
    /// let label = api.get_label("12345", None, Some("albums"), Some(10), Some(0)).await?;
    /// // Gets label info with up to 10 associated albums
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_label(
        &self,
        label_id: &str,
        with_auth: Option<bool>,
        extra: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Label, QobuzApiError> {
        let mut params = vec![("label_id".to_string(), label_id.to_string())];

        if let Some(extra_val) = extra {
            params.push(("extra".to_string(), extra_val.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(25).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        let _use_auth = with_auth.unwrap_or(false);

        self.get("/label/get", &params).await
    }
}

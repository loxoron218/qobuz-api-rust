use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError::{self, QobuzApiInitializationError},
    models::{QobuzApiStatusResponse, UserFavorites, UserFavoritesIds},
};

impl QobuzApiService {
    /// Add tracks, albums & artists to the authenticated user's favorites.
    /// At least 1 type of favorite to add is required as parameter.
    ///
    /// # Arguments
    /// * `track_ids` - IDs of the tracks to add, comma separated list (optional)
    /// * `album_ids` - IDs of the albums to add, comma separated list (optional)
    /// * `artist_ids` - IDs of the artists to add, comma separated list (optional)
    ///
    /// # Returns
    /// * `Ok(QobuzApiStatusResponse)` - Response indicating if the request was successful
    /// * `Err(QobuzApiError)` - If the API request fails
    ///
    /// # Example
    /// ```rust,no_run
    /// # use qobuz_api_rust::QobuzApiService;
    /// # use tokio;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let service = QobuzApiService::new().await?;
    /// // Add a track to favorites
    /// let response = service.add_user_favorites(
    ///     Some("123456789"),
    ///     None,
    ///     None,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_user_favorites(
        &self,
        track_ids: Option<&str>,
        album_ids: Option<&str>,
        artist_ids: Option<&str>,
    ) -> Result<QobuzApiStatusResponse, QobuzApiError> {
        let mut params = Vec::new();

        if let Some(ids) = track_ids {
            params.push(("track_ids".to_string(), ids.to_string()));
        }

        if let Some(ids) = album_ids {
            params.push(("album_ids".to_string(), ids.to_string()));
        }

        if let Some(ids) = artist_ids {
            params.push(("artist_ids".to_string(), ids.to_string()));
        }

        // At least one type of favorite must be provided
        if params.is_empty() {
            return Err(QobuzApiInitializationError {
                message: "At least one type of favorite (track_ids, album_ids, or artist_ids) must be provided".to_string(),
            });
        }

        self.signed_get("/favorite/create", &params).await
    }

    /// Removes tracks, albums & artists from the authenticated user's favorites.
    /// At least 1 type of favorite to remove is required as parameter.
    ///
    /// # Arguments
    /// * `track_ids` - IDs of the tracks to remove, comma separated list (optional)
    /// * `album_ids` - IDs of the albums to remove, comma separated list (optional)
    /// * `artist_ids` - IDs of the artists to remove, comma separated list (optional)
    ///
    /// # Returns
    /// * `Ok(QobuzApiStatusResponse)` - Response indicating if the request was successful
    /// * `Err(QobuzApiError)` - If the API request fails
    ///
    /// # Example
    /// ```rust,no_run
    /// # use qobuz_api_rust::QobuzApiService;
    /// # use tokio;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let service = QobuzApiService::new().await?;
    /// // Remove a track from favorites
    /// let response = service.delete_user_favorites(
    ///     Some("123456789"),
    ///     None,
    ///     None,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_user_favorites(
        &self,
        track_ids: Option<&str>,
        album_ids: Option<&str>,
        artist_ids: Option<&str>,
    ) -> Result<QobuzApiStatusResponse, QobuzApiError> {
        let mut params = Vec::new();

        if let Some(ids) = track_ids {
            params.push(("track_ids".to_string(), ids.to_string()));
        }

        if let Some(ids) = album_ids {
            params.push(("album_ids".to_string(), ids.to_string()));
        }

        if let Some(ids) = artist_ids {
            params.push(("artist_ids".to_string(), ids.to_string()));
        }

        // At least one type of favorite must be provided
        if params.is_empty() {
            return Err(QobuzApiInitializationError {
                message: "At least one type of favorite (track_ids, album_ids, or artist_ids) must be provided".to_string(),
            });
        }

        self.signed_get("/favorite/delete", &params).await
    }

    /// Gets the IDs of the user favorites for the authenticated user or user with the specified user ID.
    ///
    /// # Arguments
    /// * `user_id` - The User ID to fetch the favorites from. If omitted, returns favorites of the logged in user using user_auth_token (optional)
    /// * `limit` - The maximum number of extra results to return. Defaults to 5000, minimum 1, maximum 99999 (optional)
    /// * `offset` - The offset of the first extra result to return. Defaults to 0 (optional)
    ///
    /// # Returns
    /// * `Ok(UserFavoritesIds)` - The IDs of the user favorites
    /// * `Err(QobuzApiError)` - If the API request fails
    ///
    /// # Example
    /// ```rust,no_run
    /// # use qobuz_api_rust::QobuzApiService;
    /// # use tokio;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let service = QobuzApiService::new().await?;
    /// // Get favorite IDs with default parameters
    /// let favorites = service.get_user_favorite_ids(None, None, None).await?;
    ///
    /// // Get favorite IDs with custom parameters
    /// let favorites = service.get_user_favorite_ids(
    ///     Some("123456789"),
    ///     Some(100),
    ///     Some(50),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_favorite_ids(
        &self,
        user_id: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<UserFavoritesIds, QobuzApiError> {
        let mut params = Vec::new();

        if let Some(id) = user_id {
            params.push(("user_id".to_string(), id.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(5000).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        self.signed_get("/favorite/getUserFavoriteIds", &params)
            .await
    }

    /// Gets user favorites of the authenticated user or user with the specified user ID.
    ///
    /// # Arguments
    /// * `user_id` - The User ID to fetch the favorites from. If omitted, returns favorites of the logged in user using user_auth_token (optional)
    /// * `type_param` - Type of favorites to include in the response (optional). Possible values are 'tracks', 'albums', 'artists' & 'articles'. If no type defined, all types are returned
    /// * `limit` - The maximum number of extra results to return. Defaults to 50, minimum 1, maximum 500 (optional)
    /// * `offset` - The offset of the first extra result to return. Defaults to 0 (optional)
    ///
    /// # Returns
    /// * `Ok(UserFavorites)` - The user favorites
    /// * `Err(QobuzApiError)` - If the API request fails
    ///
    /// # Example
    /// ```rust,no_run
    /// # use qobuz_api_rust::QobuzApiService;
    /// # use tokio;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let service = QobuzApiService::new().await?;
    /// // Get all favorite types with default parameters
    /// let favorites = service.get_user_favorites(None, None, None, None).await?;
    ///
    /// // Get only favorite albums with custom parameters
    /// let album_favorites = service.get_user_favorites(
    ///     None,
    ///     Some("albums"),
    ///     Some(100),
    ///     Some(20),
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_favorites(
        &self,
        user_id: Option<&str>,
        type_param: Option<&str>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<UserFavorites, QobuzApiError> {
        let mut params = Vec::new();

        if let Some(id) = user_id {
            params.push(("user_id".to_string(), id.to_string()));
        }

        if let Some(t) = type_param {
            params.push(("type".to_string(), t.to_string()));
        }

        params.push(("limit".to_string(), limit.unwrap_or(50).to_string()));
        params.push(("offset".to_string(), offset.unwrap_or(0).to_string()));

        self.signed_get("/favorite/getUserFavorites", &params).await
    }
}

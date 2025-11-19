use crate::{
    api::service::QobuzApiService,
    errors::QobuzApiError,
    models::{Login, QobuzApiStatusResponse},
};

impl QobuzApiService {
    /// Internal helper to update the user authentication token in the service instance.
    ///
    /// This method extracts the authentication token from the login response and stores it
    /// in the service for use in subsequent API requests that require authentication.
    ///
    /// # Arguments
    ///
    /// * `result` - A reference to the login response containing the authentication token
    ///
    /// # Returns
    ///
    /// A clone of the original login response
    fn update_auth_token(&mut self, result: &Login) -> Login {
        // Update the user auth token in the service
        if let Some(auth_token) = &result.auth_token {
            self.user_auth_token = Some(auth_token.clone());
        }
        result.clone()
    }

    /// Authenticates a user with the Qobuz API using their identifier and password.
    ///
    /// This method performs a login request to the Qobuz API using either an email address
    /// or username as the identifier. The password must be provided as an MD5 hash.
    /// On successful login, the user authentication token is automatically stored in the
    /// service instance for use in subsequent authenticated API requests.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The user's identifier, which can be either an email address or username
    /// * `password` - The MD5 hash of the user's password
    ///
    /// # Returns
    ///
    /// * `Ok(Login)` - A login response containing user information and authentication token
    /// * `Err(QobuzApiError)` - If the API request fails or authentication is unsuccessful
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
    /// let mut api = QobuzApiService::new().await?;
    /// // Note: Password should be MD5 hashed
    /// let login_result = api.login("user@example.com", "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login(
        &mut self,
        identifier: &str,
        password: &str,
    ) -> Result<Login, QobuzApiError> {
        let params = vec![
            ("username".to_string(), identifier.to_string()), // Qobuz API uses "username" field for both email and username
            ("password".to_string(), password.to_string()),
        ];

        let result: Login = self.post("/user/login", &params).await?;
        let result = self.update_auth_token(&result);

        Ok(result)
    }

    /// Authenticates a user with the Qobuz API using their user ID and authentication token.
    ///
    /// This method allows authentication using an existing user ID and authentication token,
    /// which can be useful for maintaining sessions across application restarts.
    /// On successful login, the authentication token is stored in the service instance
    /// for use in subsequent API requests that require authentication.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's unique identifier in the Qobuz system
    /// * `user_auth_token` - The user's authentication token
    ///
    /// # Returns
    ///
    /// * `Ok(Login)` - A login response containing user information and authentication token
    /// * `Err(QobuzApiError)` - If the API request fails or authentication is unsuccessful
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
    /// let mut api = QobuzApiService::new().await?;
    /// let login_result = api.login_with_token("123456789", "auth_token_here").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login_with_token(
        &mut self,
        user_id: &str,
        user_auth_token: &str,
    ) -> Result<Login, QobuzApiError> {
        let params = vec![
            ("user_id".to_string(), user_id.to_string()),
            ("user_auth_token".to_string(), user_auth_token.to_string()),
        ];

        let result: Login = self.post("/user/login", &params).await?;
        let result = self.update_auth_token(&result);

        Ok(result)
    }

    /// Requests a password reset link for the specified user identifier.
    ///
    /// This method sends a password reset request to the Qobuz API for the given identifier,
    /// which can be either an email address or username. If the identifier exists in the system,
    /// the user will receive instructions to reset their password.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The user's identifier (email address or username) for which to request a password reset
    ///
    /// # Returns
    ///
    /// * `Ok(QobuzApiStatusResponse)` - A response indicating whether the password reset request was successful
    /// * `Err(QobuzApiError)` - If the API request fails
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use qobuz_api_rust::{QobuzApiService, QobuzApiError};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), qobuz_api_rust::QobuzApiError> {
    /// let api = QobuzApiService::new().await?;
    /// let result = api.reset_password("user@example.com").await?;
    /// if result.status == Some("success".to_string()) {
    ///     println!("Password reset email sent successfully");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reset_password(
        &self,
        identifier: &str,
    ) -> Result<QobuzApiStatusResponse, QobuzApiError> {
        let params = vec![("username".to_string(), identifier.to_string())];

        let result: QobuzApiStatusResponse = self.get("/user/resetPassword", &params).await?;
        Ok(result)
    }
}

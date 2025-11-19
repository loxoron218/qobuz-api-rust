use {
    serde::de::DeserializeOwned,
    serde_json::{Value, from_value},
};

use crate::{
    api::service::constants,
    errors::QobuzApiError::{self, ApiErrorResponse, ApiResponseParseError, HttpError},
    models::QobuzApiStatusResponse,
    utils::{deserialize_response, get_current_timestamp, get_md5_hash, to_query_string},
};

impl crate::api::service::QobuzApiService {
    /// Sends a GET request to the Qobuz API.
    ///
    /// This method handles the complete request lifecycle including parameter formatting,
    /// authentication token injection, response parsing, and error handling. It automatically
    /// includes common parameters and checks for API error responses.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (e.g., "/album/get")
    /// * `params` - A slice of key-value parameter pairs to include in the query string
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that must implement `DeserializeOwned`
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized response data if the request is successful,
    /// or `Err(QobuzApiError)` if the request fails due to network issues, API errors,
    /// or response parsing problems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use qobuz_api_rust::api::service::QobuzApiService;
    /// # use qobuz_api_rust::models::Album;
    /// # async fn example(service: &QobuzApiService) -> Result<(), Box<dyn std::error::Error>> {
    /// let album_id = "12345";
    /// let params = vec![("album_id".to_string(), album_id.to_string())];
    /// let album: Album = service.get("/album/get", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request fails (network issues)
    /// - The API returns an error response (status: "error")
    /// - The response cannot be parsed as the expected type `T`
    /// - The response cannot be deserialized from JSON
    pub async fn get<T>(
        &self,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<T, QobuzApiError>
    where
        T: DeserializeOwned,
    {
        // Add common parameters
        let all_params = params.to_vec();

        let query_string = to_query_string(&all_params);
        let url = format!("{}{}?{}", constants::API_BASE_URL, endpoint, query_string);

        let mut request = self.client.get(&url);

        if let Some(ref token) = self.user_auth_token {
            request = request.header("X-User-Auth-Token", token);
        }

        let response = request.send().await.map_err(HttpError)?;
        let value: Value = deserialize_response(response).await?;

        if let Some(status) = value.get("status")
            && status == "error"
        {
            let error_response: QobuzApiStatusResponse =
                from_value(value.clone()).map_err(|e| ApiResponseParseError {
                    content: e.to_string(),
                    source: e,
                })?;

            return Err(ApiErrorResponse {
                code: error_response.code.unwrap_or_default(),
                message: error_response.message.unwrap_or_default(),
                status: error_response.status.unwrap_or_default(),
            });
        }

        from_value(value).map_err(|e| ApiResponseParseError {
            content: e.to_string(),
            source: e,
        })
    }

    /// Sends a POST request to the Qobuz API.
    ///
    /// This method handles POST requests to the Qobuz API, automatically including
    /// required parameters like the application ID and user authentication token if available.
    /// It manages the complete request lifecycle including parameter formatting,
    /// response parsing, and error handling.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (e.g., "/user/login")
    /// * `params` - A slice of key-value parameter pairs to include in the form body
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that must implement `DeserializeOwned`
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized response data if the request is successful,
    /// or `Err(QobuzApiError)` if the request fails due to network issues, API errors,
    /// or response parsing problems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use qobuz_api_rust::api::service::QobuzApiService;
    /// # use qobuz_api_rust::models::Login;
    /// # async fn example(service: &QobuzApiService) -> Result<(), Box<dyn std::error::Error>> {
    /// let params = vec![
    ///     ("email".to_string(), "user@example.com".to_string()),
    ///     ("password".to_string(), "hashed_password".to_string()),
    /// ];
    /// let login_response: Login = service.post("/user/login", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request fails (network issues)
    /// - The API returns an error response (status: "error")
    /// - The response cannot be parsed as the expected type `T`
    /// - The response cannot be deserialized from JSON
    pub async fn post<T>(
        &self,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<T, QobuzApiError>
    where
        T: DeserializeOwned,
    {
        // Add common parameters
        let mut all_params = params.to_vec();
        all_params.push(("app_id".to_string(), self.app_id.clone()));

        if let Some(ref token) = self.user_auth_token {
            all_params.push(("user_auth_token".to_string(), token.clone()));
        }

        let url = format!("{}{}", constants::API_BASE_URL, endpoint);

        let response = self
            .client
            .post(&url)
            .form(&all_params)
            .send()
            .await
            .map_err(HttpError)?;
        let value: Value = deserialize_response(response).await?;

        if let Some(status) = value.get("status")
            && status == "error"
        {
            let error_response: QobuzApiStatusResponse =
                from_value(value).map_err(|e| ApiResponseParseError {
                    content: e.to_string(),
                    source: e,
                })?;
            return Err(ApiErrorResponse {
                code: error_response.code.unwrap_or_default(),
                message: error_response.message.unwrap_or_default(),
                status: error_response.status.unwrap_or_default(),
            });
        }

        from_value(value).map_err(|e| ApiResponseParseError {
            content: e.to_string(),
            source: e,
        })
    }

    /// Generates a signature for protected Qobuz API endpoints.
    ///
    /// This method creates a signature string using the Qobuz API's authentication scheme.
    /// The signature is computed by concatenating the HTTP method, endpoint, sorted parameters,
    /// and application secret, then applying an MD5 hash. This ensures the request is
    /// authenticated and authorized.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method (e.g., "GET", "POST")
    /// * `endpoint` - The API endpoint to call (e.g., "/album/get")
    /// * `params` - A slice of key-value parameter pairs to include in the signature calculation
    ///
    /// # Returns
    ///
    /// Returns a string containing the MD5 hash of the signature string.
    ///
    /// # Algorithm
    ///
    /// The signature is generated by:
    /// 1. Adding common parameters (app_id, method, timestamp, user_auth_token if present)
    /// 2. Sorting parameters alphabetically by key
    /// 3. Creating a signature string by concatenating method, endpoint, sorted parameters, and app secret
    /// 4. Computing the MD5 hash of the signature string
    fn generate_signature(
        &self,
        method: &str,
        endpoint: &str,
        params: &[(String, String)],
    ) -> String {
        let timestamp = get_current_timestamp();
        let mut all_params = params.to_vec();
        all_params.push(("app_id".to_string(), self.app_id.clone()));
        all_params.push(("method".to_string(), method.to_string()));
        all_params.push(("timestamp".to_string(), timestamp.clone()));

        if let Some(ref token) = self.user_auth_token {
            all_params.push(("user_auth_token".to_string(), token.clone()));
        }

        // Sort parameters alphabetically by key
        all_params.sort_by(|a, b| a.0.cmp(&b.0));

        // Create the signature string
        let mut signature_string = format!("{}{}", method, endpoint);
        for (key, value) in &all_params {
            signature_string.push_str(&format!("{}{}", key, value));
        }
        signature_string.push_str(&self.app_secret);

        get_md5_hash(&signature_string)
    }

    /// Sends a GET request to the Qobuz API with signature authentication.
    ///
    /// This method is used for protected endpoints that require signature-based authentication.
    /// It automatically generates the required signature using the Qobuz API's authentication
    /// scheme, includes the necessary parameters (app_id, user_auth_token if available),
    /// and handles the complete request lifecycle including response parsing and error handling.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (e.g., "/album/get")
    /// * `params` - A slice of key-value parameter pairs to include in the query string
    ///
    /// # Type Parameters
    ///
    /// * `T` - The expected response type that must implement `DeserializeOwned`
    ///
    /// # Returns
    ///
    /// Returns `Ok(T)` with the deserialized response data if the request is successful,
    /// or `Err(QobuzApiError)` if the request fails due to network issues, API errors,
    /// or response parsing problems.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use qobuz_api_rust::api::service::QobuzApiService;
    /// # use qobuz_api_rust::models::Album;
    /// # async fn example(service: &QobuzApiService) -> Result<(), Box<dyn std::error::Error>> {
    /// let album_id = "12345";
    /// let params = vec![("album_id".to_string(), album_id.to_string())];
    /// let album: Album = service.signed_get("/album/get", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request fails (network issues)
    /// - The API returns an error response (status: "error")
    /// - The response cannot be parsed as the expected type `T`
    /// - The response cannot be deserialized from JSON
    pub async fn signed_get<T>(
        &self,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<T, QobuzApiError>
    where
        T: DeserializeOwned,
    {
        // Add common parameters
        let mut all_params = params.to_vec();
        all_params.push(("app_id".to_string(), self.app_id.clone()));

        if let Some(ref token) = self.user_auth_token {
            all_params.push(("user_auth_token".to_string(), token.clone()));
        }

        // Generate signature
        let signature = self.generate_signature("GET", endpoint, params);
        all_params.push(("request_ts".to_string(), get_current_timestamp()));
        all_params.push(("request_sig".to_string(), signature));

        let query_string = to_query_string(&all_params);
        let url = format!("{}{}?{}", constants::API_BASE_URL, endpoint, query_string);

        let response = self.client.get(&url).send().await.map_err(HttpError)?;
        let value: Value = deserialize_response(response).await?;

        if let Some(status) = value.get("status")
            && status == "error"
        {
            let error_response: QobuzApiStatusResponse =
                from_value(value).map_err(|e| ApiResponseParseError {
                    content: e.to_string(),
                    source: e,
                })?;
            return Err(ApiErrorResponse {
                code: error_response.code.unwrap_or_default(),
                message: error_response.message.unwrap_or_default(),
                status: error_response.status.unwrap_or_default(),
            });
        }

        from_value(value).map_err(|e| ApiResponseParseError {
            content: e.to_string(),
            source: e,
        })
    }
}

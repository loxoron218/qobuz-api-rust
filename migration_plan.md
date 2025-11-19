# Qobuz API Library Migration: C# to Rust

This document outlines the step-by-step plan to migrate the C# Qobuz API library to Rust, preserving all functionality while adhering to Rust's best practices.

## Overview

The C# Qobuz API library provides access to Qobuz's music streaming API, including authentication, content retrieval (albums, artists, tracks), search functionality, user favorites management, and streaming URL generation. The migration to Rust will maintain the same functionality while leveraging Rust's memory safety, performance, and concurrency features.

## Migration Steps

### Phase 1: Project Setup and Dependencies

1. **Initialize Rust Project**
   - Create a new Rust library project: `cargo new qobuz-api-rust`
   - Configure `Cargo.toml` with necessary dependencies
   - **Relevant C# files:** `QobuzApiSharp/QobuzApiSharp.csproj`

2. **Configure Dependencies**
   - `reqwest` for HTTP client functionality
   - `tokio` for async runtime
   - `serde` and `serde_json` for serialization/deserialization
   - `md5` for MD5 hashing
   - `regex` for regex operations
   - `thiserror` for error handling
   - `chrono` for date/time handling
   - **Relevant C# files:** `QobuzApiSharp/QobuzApiSharp.csproj`, `QobuzApiSharp/packages.config`

### Phase 2: Core Data Structures

3. **Create Error Types**
   - Define error enum using `thiserror`:
     - `ApiErrorResponse { code: String, message: String, status: String }`
     - `ApiResponseParseError { content: String, source: Box<dyn std::error::Error> }`
     - `QobuzApiInitializationError { message: String }`
   - **Relevant C# files:** `QobuzApiSharp/Api/Exceptions/ApiErrorResponseException.cs`, `QobuzApiSharp/Api/Exceptions/ApiResponseParseErrorException.cs`, `QobuzApiSharp/Api/Exceptions/QobuzApiInitializationException.cs`

4. **Create Basic Models**
   - `QobuzApiStatusResponse` with `code`, `message`, and `status` fields
   - Basic content models (Artist, Album, Track, User, etc.)
   - **Relevant C# files:** `QobuzApiSharp/Api/Models/QobuzApiStatusResponse.cs`, `QobuzApiSharp/Api/Models/Content/General/Artist.cs`, `QobuzApiSharp/Api/Models/Content/Album/Album.cs`, `QobuzApiSharp/Api/Models/Content/Track/Track.cs`, `QobuzApiSharp/Api/Models/User/User.cs`

5. **Implement JSON Serialization**
   - Use `serde` derive macros for all models
   - Implement custom deserializers where needed (similar to C# converters)
   - **Relevant C# files:** `QobuzApiSharp/Api/Converters/TrackIdsConverter.cs`, `QobuzApiSharp/Api/Converters/MostPopularContentConverter.cs`, `QobuzApiSharp/Api/Service/QobuzApiHelper.cs`

### Phase 3: Utilities and Helpers

6. **Create Utilities Module**
   - MD5 utilities for hashing
   - Query string builder
   - Time utilities for Unix timestamps
   - **Relevant C# files:** `QobuzApiSharp/Api/Utilities/MD5Utilities.cs`

7. **Implement API Helper Functions**
   - Web player app ID/secret extraction
   - File URL signature generation
   - HTTP response deserialization
   - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiHelper.cs`, `QobuzApiSharp/Api/QobuzApiConstants.cs`

### Phase 4: HTTP Client Implementation

8. **Create Qobuz API Service Structure**
   - Define `QobuzApiService` struct with:
     - `app_id: String`
     - `app_secret: String`
     - `user_auth_token: Option<String>`
     - `client: reqwest::Client`
   - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiService.cs`

9. **Implement HTTP Request Handling**
   - Async methods for sending requests
   - Error handling for API responses
   - Request signature generation for protected endpoints
   - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiService.cs`, `QobuzApiSharp/Api/Service/QobuzApiHelper.cs`

### Phase 5: Authentication System

10. **Implement User Authentication**
    - `login_with_email` method
    - `login_with_username` method
    - `login_with_token` method
    - `reset_password` methods
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.User.cs`, `QobuzApiSharp/Api/Models/User/Login.cs`

11. **Session Management**
    - Token storage and retrieval
    - Authentication header management
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiService.cs`, `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.User.cs`

### Phase 6: Content Endpoints

12. **Album Endpoints**
    - `get_album` method
    - `search_albums` method
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Album.cs`, `QobuzApiSharp/Api/Models/Content/Album/Album.cs`

13. **Artist Endpoints**
    - `get_artist` method
    - `search_artists` method
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Artist.cs`, `QobuzApiSharp/Api/Models/Content/General/Artist.cs`

14. **Track Endpoints**
    - `get_track` method
    - `get_track_file_url` method (with signature generation)
    - `search_tracks` method
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Track.cs`, `QobuzApiSharp/Api/Models/Content/Track/Track.cs`, `QobuzApiSharp/Api/Models/Content/Track/FileUrl.cs`

15. **Playlist Endpoints**
    - `get_playlist` method
    - `search_playlists` method
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Playlist.cs`, `QobuzApiSharp/Api/Models/Content/Playlist/Playlist.cs`

16. **Other Content Endpoints**
    - Article endpoints
    - Label endpoints
    - Story endpoints
    - Catalog search
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Article.cs`, `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Label.cs`, `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Story.cs`, `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Catalog.cs`

### Phase 7: User Functionality

17. **Favorite Management**
    - `add_user_favorites` method
    - `delete_user_favorites` method
    - `get_user_favorite_ids` method
    - `get_user_favorites` method
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/Endpoints/QobuzApiService.Favorite.cs`, `QobuzApiSharp/Api/Models/Content/General/UserFavorites.cs`, `QobuzApiSharp/Api/Models/Content/General/UserFavoritesIds.cs`

18. **Additional User Features**
    - User profile retrieval
    - Subscription information
    - **Relevant C# files:** `QobuzApiSharp/Api/Models/User/User.cs`, `QobuzApiSharp/Api/Models/User/Subscription.cs`

### Phase 8: Advanced Features

19. **Web Player Integration**
    - Implement fetching app ID/secret from web player
    - Bundle.js parsing with regex
    - Base64 decoding and string manipulation
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiHelper.cs`, `QobuzApiSharp/Api/QobuzApiConstants.cs`

20. **Specialized Converters**
    - Implement custom deserializers for inconsistent API responses
    - Handle the TrackIdsConverter functionality for mixed JSON types
    - **Relevant C# files:** `QobuzApiSharp/Api/Converters/TrackIdsConverter.cs`, `QobuzApiSharp/Api/Converters/MostPopularContentConverter.cs`

### Phase 9: Testing and Validation

21. **Unit Tests**
    - Test all model serialization/deserialization
    - Test utility functions
    - Test API helper functions
    - **Relevant C# files:** All model files, utility files, and helper files

22. **Integration Tests**
    - Test API endpoints with mock responses
    - Test authentication flows
    - Test error handling
    - **Relevant C# files:** All endpoint files in `QobuzApiSharp/Api/Service/Endpoints/`

23. **Functional Verification**
    - Verify all functionality matches C# implementation
    - Test with real Qobuz API (if possible with test credentials)
    - **Relevant C# files:** All files in the codebase for reference

### Phase 10: Documentation and Best Practices

24. **Documentation**
    - Add comprehensive Rust documentation comments
    - Create examples for common use cases
    - Document error handling patterns
    - **Relevant C# files:** All files in the codebase for reference

25. **Rust Best Practices Implementation**
    - Proper error handling with `Result<T, E>` types
    - Memory safety without garbage collection
    - Use of async/await for I/O operations
    - Proper ownership and borrowing patterns
    - Implement `Display` and `Debug` traits where appropriate
    - **Relevant C# files:** All files in the codebase for reference

26. **Performance Optimizations**
    - Connection pooling with reqwest
    - Efficient JSON parsing
    - Proper async execution patterns
    - **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiService.cs`, `QobuzApiSharp/Api/Service/QobuzApiHelper.cs`

## Rust-Specific Considerations

### Memory Safety
- Use `String` instead of `&str` for owned data in structs
- Implement proper ownership patterns
- Avoid unnecessary cloning
- **Relevant C# files:** All model files for reference

### Async Programming
- Use `tokio::task` for concurrent operations
- Implement proper async/await patterns
- Handle async error propagation correctly
- **Relevant C# files:** `QobuzApiSharp/Api/Service/QobuzApiService.cs`

### Error Handling
- Use `thiserror` for comprehensive error types
- Implement proper error chaining
- Provide meaningful error messages
- **Relevant C# files:** `QobuzApiSharp/Api/Exceptions/ApiErrorResponseException.cs`, `QobuzApiSharp/Api/Exceptions/ApiResponseParseErrorException.cs`, `QobuzApiSharp/Api/Exceptions/QobuzApiInitializationException.cs`

### Type Safety
- Leverage Rust's type system for API safety
- Use `Option<T>` for nullable fields
- Use `Result<T, E>` for fallible operations
- Implement proper validation for API inputs
- **Relevant C# files:** All model files for reference

## API Compatibility

The Rust implementation will maintain the same public API contract as the C# version:
- Same method names and signatures (adapted to Rust conventions)
- Same parameter types and return types
- Same error handling patterns
- Same authentication mechanisms
- Same functionality for all endpoints
- **Relevant C# files:** All endpoint files in `QobuzApiSharp/Api/Service/Endpoints/`

## Testing Strategy

- Implement comprehensive unit tests for all modules
- Use mock HTTP servers for API testing
- Test error scenarios and edge cases
- Verify JSON serialization/deserialization correctness
- Performance testing to ensure Rust benefits are realized
- **Relevant C# files:** All files in the codebase for reference

## Deployment and Distribution

- Package as a Rust crate on crates.io
- Provide clear usage examples
- Document installation and setup requirements
- Include compatibility notes for different Rust versions
- **Relevant C# files:** `QobuzApiSharp/QobuzApiSharp.csproj`, `QobuzApiSharp/QobuzApiSharp.nuspec`
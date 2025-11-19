/// Qobuz API data models
///
/// This module contains all the data structures used to represent Qobuz API responses.
/// These models are used for deserializing JSON responses from the API into Rust structs.
/// The models cover all major Qobuz content types including albums, artists, tracks,
/// playlists, users, and various metadata fields.
pub mod album;
/// Article, author, biography, and story models
///
/// This module contains models for articles, authors, biographies, and stories
/// from the Qobuz platform, including their metadata and relationships.
pub mod article;
/// Artist model containing comprehensive artist information
///
/// This module contains the Artist struct which represents an artist on the Qobuz platform
/// with their identification, name, profile picture, album counts, roles, and related content.
pub mod artist;
/// Core models for API responses and authentication
///
/// This module contains fundamental models for API responses, including status responses
/// and login information, as well as utility functions for deserialization.
pub mod core;
/// Credential model containing user credential information
///
/// This module contains the Credential struct which represents comprehensive user
/// credential information including personal details, account settings, and feature availability.
pub mod credential;
/// Metadata models for images, audio info, genres, labels, and other metadata
///
/// This module contains various models for handling metadata such as images, audio information,
/// genres, labels, tags, areas, awards, and other related metadata used in the Qobuz API.
pub mod metadata;
/// Playlist model containing information about user playlists
///
/// This module contains the Playlist struct which represents a playlist with details about
/// its content, owner, creation date, and various properties.
pub mod playlist;
/// Release models containing information about music releases
///
/// This module contains models for music releases including release details, tracks,
/// artists, physical support, rights, and audio information.
pub mod release;
/// Search models for search results and related content
///
/// This module contains models for search results across different content types
/// including albums, articles, artists, playlists, tracks, and user favorites.
pub mod search;
/// Subscription models containing user subscription information
///
/// This module contains models for user subscription details including plan information,
/// status, dates, and payment information.
pub mod subscription;
/// Track model containing comprehensive track information
///
/// This module contains the Track struct which represents a track on the Qobuz platform
/// with its identification, title, version, duration, album, artists, and various metadata.
pub mod track;
/// Simplified user model containing basic user information
///
/// This module contains the User struct which represents a Qobuz user with their
/// identification, personal information, account details, and subscription information.
pub mod user_simple;

/// Re-exports of commonly used models for convenience
///
/// This section re-exports the most important data models from the submodules
/// to provide a convenient and streamlined API for users. These re-exports allow
/// direct access to the core models without having to specify full module paths.
pub use {
    album::Album,
    article::{Article, Author, Biography, Story},
    artist::Artist,
    core::{Login, QobuzApiStatusResponse, deserialize_code},
    credential::Credential,
    metadata::{Area, AudioInfo, Award, Focus, Genre, GenreTag, Goody, Image, Label, Period, Tag},
    playlist::Playlist,
    release::{
        FileUrl, Release, ReleaseArtist, ReleaseAudioInfo, ReleasePhysicalSupport, ReleaseRights,
        ReleaseTrack, ReleaseTrackList, ReleasesList,
    },
    search::{
        AlbumsSameArtist, ItemSearchResult, MostPopular, MostPopularContent, SearchResult,
        UserFavorites, UserFavoritesIds,
    },
    subscription::{LastUpdate, StoreFeatures, Subscription},
    track::Track,
    user_simple::User,
};

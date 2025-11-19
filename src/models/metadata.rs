use serde::{Deserialize, Serialize};

/// Image model containing URLs for different sizes of an image
///
/// This struct provides URLs for various sizes of an image, commonly used for
/// album artwork or artist photos.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Image;
///
/// let image = Image {
///     small: Some("https://example.com/small.jpg".to_string()),
///     large: Some("https://example.com/large.jpg".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Image {
    /// URL for the small version of the image
    #[serde(rename = "small")]
    pub small: Option<String>,

    /// URL for the thumbnail version of the image
    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,

    /// URL for the medium version of the image
    #[serde(rename = "medium")]
    pub medium: Option<String>,

    /// URL for the large version of the image
    #[serde(rename = "large")]
    pub large: Option<String>,

    /// URL for the extra-large version of the image
    #[serde(rename = "extralarge")]
    pub extralarge: Option<String>,

    /// URL for the mega version of the image
    #[serde(rename = "mega")]
    pub mega: Option<String>,

    /// URL for the back cover version of the image
    #[serde(rename = "back")]
    pub back: Option<String>,
}

/// Audio information model containing replay gain data
///
/// This struct contains audio-specific information such as replay gain values
/// that help normalize playback volume across different tracks.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::AudioInfo;
///
/// let audio_info = AudioInfo {
///     replaygain_track_peak: Some(0.98),
///     replaygain_track_gain: Some(-2.5),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AudioInfo {
    /// Peak amplitude value for replay gain normalization
    #[serde(rename = "replaygain_track_peak")]
    pub replaygain_track_peak: Option<f64>,

    /// Gain value in dB for replay gain normalization
    #[serde(rename = "replaygain_track_gain")]
    pub replaygain_track_gain: Option<f64>,
}

/// Genre model containing information about a musical genre
///
/// This struct represents a musical genre with its identification, name, slug,
/// path in the genre hierarchy, and color coding.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Genre;
///
/// let genre = Genre {
///     id: Some(123),
///     name: Some("Classical".to_string()),
///     slug: Some("classical".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Genre {
    /// Unique identifier for the genre
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the genre
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the genre
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// Path in the genre hierarchy as a list of genre IDs
    #[serde(rename = "path")]
    pub path: Option<Vec<i32>>,

    /// Color associated with the genre for UI purposes
    #[serde(rename = "color")]
    pub color: Option<String>,
}

/// Label model containing information about a record label
///
/// This struct represents a record label with its identification, name, and slug.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Label;
///
/// let label = Label {
///     id: Some(456),
///     name: Some("Example Records".to_string()),
///     slug: Some("example-records".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Label {
    /// Unique identifier for the label
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the label
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the label
    #[serde(rename = "slug")]
    pub slug: Option<String>,
}

/// Tag model containing information about a tag
///
/// This struct represents a tag with its identification, name, slug, color,
/// and various properties related to discovery and genre classification.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Tag;
///
/// let tag = Tag {
///     id: Some(123),
///     name: Some("Jazz".to_string()),
///     slug: Some("jazz".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tag {
    /// Unique identifier for the tag
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the tag
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Name of the tag in JSON format
    #[serde(rename = "name_json")]
    pub name_json: Option<String>,

    /// URL-friendly slug for the tag
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// Color associated with the tag for UI purposes
    #[serde(rename = "color")]
    pub color: Option<String>,

    /// Whether the tag is a discovery tag
    #[serde(rename = "is_discover")]
    pub is_discover: Option<bool>,

    /// ID of the featured tag
    #[serde(rename = "featured_tag_id")]
    pub featured_tag_id: Option<String>,

    /// Genre tag information for the tag
    #[serde(rename = "genre_tag")]
    pub genre_tag: Option<GenreTag>,
}

/// Genre tag model containing information about a genre tag
///
/// This struct represents a genre tag with its genre ID, name, and slug.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::GenreTag;
///
/// let genre_tag = GenreTag {
///     genre_id: Some("123".to_string()),
///     name: Some("Rock".to_string()),
///     slug: Some("rock".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenreTag {
    /// ID of the associated genre
    #[serde(rename = "genre_id")]
    pub genre_id: Option<String>,

    /// Name of the genre tag
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the genre tag
    #[serde(rename = "slug")]
    pub slug: Option<String>,
}

/// Area model containing information about a geographical area
///
/// This struct represents a geographical area with its identification, name, and slug.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Area;
///
/// let area = Area {
///     id: Some(123),
///     name: Some("United States".to_string()),
///     slug: Some("united-states".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Area {
    /// Unique identifier for the area
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the area
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the area
    #[serde(rename = "slug")]
    pub slug: Option<String>,
}

/// Award model containing information about an award
///
/// This struct represents an award with its identification, name, year, and description.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Award;
///
/// let award = Award {
///     id: Some(456),
///     name: Some("Best Album".to_string()),
///     year: Some(2023),
///     description: Some("Award for the best album of the year".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Award {
    /// Unique identifier for the award
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the award
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Year when the award was given
    #[serde(rename = "year")]
    pub year: Option<i32>,

    /// Description of the award
    #[serde(rename = "description")]
    pub description: Option<String>,
}

/// Goody model containing information about bonus content
///
/// This struct represents additional content or "goodies" associated with an album,
/// such as booklets, photos, or other bonus materials.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Goody;
///
/// let goody = Goody {
///     id: Some(789),
///     title: Some("Digital Booklet".to_string()),
///     type_field: Some("booklet".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Goody {
    /// Unique identifier for the goody
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Title of the goody
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// URL to access the goody
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// Type of the goody (e.g., "booklet", "photo", "video")
    #[serde(rename = "type")]
    pub type_field: Option<String>,

    /// URL to the goody's image
    #[serde(rename = "image")]
    pub image: Option<String>,
}

/// Focus model containing information about a focused item
///
/// This struct represents a focused item with its identification and type.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Focus;
///
/// let focus = Focus {
///     id: Some("focus123".to_string()),
///     type_field: Some("album".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Focus {
    /// Unique identifier for the focus item
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Type of the focus item (e.g., "album", "artist", "track")
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// Period model containing information about a time period
///
/// This struct represents a time period with its identification, name, and slug.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Period;
///
/// let period = Period {
///     id: Some(1980),
///     name: Some("1980s".to_string()),
///     slug: Some("1980s".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Period {
    /// Unique identifier for the period
    #[serde(rename = "id")]
    pub id: Option<i32>,

    /// Name of the period
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the period
    #[serde(rename = "slug")]
    pub slug: Option<String>,
}

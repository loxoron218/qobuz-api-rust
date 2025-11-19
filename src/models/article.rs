use serde::{Deserialize, Serialize};

use crate::models::Author as AuthorModel;

/// Article model containing information about an article
///
/// This struct represents an article with its identification, title, description,
/// authors, and various metadata.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Article;
///
/// let article = Article {
///     id: Some("article123".to_string()),
///     title: Some("Music Review".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Article {
    /// Unique identifier for the article
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Title of the article
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Full description of the article
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Short description of the article
    #[serde(rename = "description_short")]
    pub description_short: Option<String>,

    /// URL to the article
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// URL to the article's main image
    #[serde(rename = "image")]
    pub image: Option<String>,

    /// URL to the article's rectangular image
    #[serde(rename = "image_rectangle")]
    pub image_rectangle: Option<String>,

    /// List of authors who wrote the article
    #[serde(rename = "authors")]
    pub authors: Option<Vec<AuthorModel>>,

    /// Display date for the article
    #[serde(rename = "display_date")]
    pub display_date: Option<String>,

    /// List of section slugs the article belongs to
    #[serde(rename = "section_slugs")]
    pub section_slugs: Option<Vec<String>>,

    /// List of tags associated with the article
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,

    /// URL-friendly slug for the article
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// Unix timestamp of when the article was created
    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,

    /// Unix timestamp of when the article was last updated
    #[serde(rename = "updated_at")]
    pub updated_at: Option<i64>,
}

/// Author model containing information about an author
///
/// This struct represents an author with their identification, name, slug, and image.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Author;
///
/// let author = Author {
///     id: Some("author123".to_string()),
///     name: Some("John Doe".to_string()),
///     slug: Some("john-doe".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Author {
    /// Unique identifier for the author
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Name of the author
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// URL-friendly slug for the author
    #[serde(rename = "slug")]
    pub slug: Option<String>,

    /// URL to the author's image
    #[serde(rename = "image")]
    pub image: Option<String>,
}

/// Story model containing information about a story or article
///
/// This struct represents a story or article with its identification, title,
/// description, authors, and various metadata.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Story;
///
/// let story = Story {
///     id: Some("story123".to_string()),
///     title: Some("Music Story".to_string()),
///     display_date: Some(1672531200), // Unix timestamp
///     ..Default::default()
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Story {
    /// Unique identifier for the story
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// List of section slugs the story belongs to
    #[serde(rename = "section_slugs")]
    pub section_slugs: Option<Vec<String>>,

    /// Title of the story
    #[serde(rename = "title")]
    pub title: Option<String>,

    /// Short description of the story
    #[serde(rename = "description_short")]
    pub description_short: Option<String>,

    /// List of authors who wrote the story
    #[serde(rename = "authors")]
    pub authors: Option<Vec<Author>>,

    /// URL to the story's main image
    #[serde(rename = "image")]
    pub image: Option<String>,

    /// Display date for the story as a Unix timestamp
    #[serde(rename = "display_date")]
    pub display_date: Option<i64>,
}

/// Biography model containing information about an artist's biography
///
/// This struct represents an artist's biography with content, summary, and source information.
///
/// # Examples
///
/// ```
/// use qobuz_api_rust::models::Biography;
///
/// let biography = Biography {
///     content: Some("Full biography content...".to_string()),
///     summary: Some("Brief summary...".to_string()),
///     source: Some("Official biography".to_string()),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Biography {
    /// Full content of the biography
    #[serde(rename = "content")]
    pub content: Option<String>,

    /// Summary of the biography
    #[serde(rename = "summary")]
    pub summary: Option<String>,

    /// Source of the biography information
    #[serde(rename = "source")]
    pub source: Option<String>,
}

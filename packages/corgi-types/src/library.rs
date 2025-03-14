use std::str::FromStr;

use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;
use uuid::Uuid;

use corgi_database::entities::{library, library_directory};

use crate::Paginated;

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Display, EnumString)]
pub enum LibraryCategory {
    #[default]
    Movie,

    TvShow,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Library {
    pub id: Uuid,

    #[schema(example = "Movies")]
    pub name: String,

    #[schema(example = "Movie")]
    pub category: LibraryCategory,

    #[schema(example = "zh")]
    pub language: String,

    #[schema(example = "CN")]
    pub region: String,

    #[schema(example = "[\"media.corgi.metadata.themoviedb\"]")]
    pub metadata_providers: Vec<String>,

    pub scanned_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl From<library::Model> for Library {
    fn from(value: library::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category: LibraryCategory::from_str(&value.category).unwrap_or_default(),
            language: value.language,
            region: value.region,
            metadata_providers: serde_json::from_value(value.metadata_providers)
                .unwrap_or_default(),
            scanned_at: value.scanned_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Paginated<library::Model>> for Paginated<Library> {
    fn from(value: Paginated<library::Model>) -> Self {
        Self {
            items: value.items.into_iter().map(Library::from).collect(),
            count: value.count,
            limit: value.limit,
            offset: value.offset,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct LibraryPayload {
    #[garde(length(min = 3, max = 128))]
    #[schema(example = "Movies")]
    pub name: String,

    #[schema(example = "Movie")]
    #[garde(skip)]
    pub category: LibraryCategory,

    #[garde(length(equal = 2), pattern("[a-z]{2}"))]
    #[schema(example = "zh")]
    pub language: String,

    #[garde(length(equal = 2), pattern("[A-Z]{2}"))]
    #[schema(example = "CN")]
    pub region: String,

    #[schema(example = "[\"media.corgi.TMDB\"]")]
    #[garde(skip)]
    pub metadata_providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LibraryDirectory {
    #[schema(example = "1")]
    pub id: i32,

    pub library_id: Uuid,

    #[schema(example = "/path/to/library")]
    pub path: String,

    pub scanned_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl From<library_directory::Model> for LibraryDirectory {
    fn from(value: library_directory::Model) -> Self {
        Self {
            id: value.id,
            library_id: value.library_id,
            path: value.path,
            scanned_at: value.scanned_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct LibraryDirectoryPayload {
    #[schema(example = "/path/to/library")]
    #[garde(skip)]
    pub path: String,
}

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub count: u64,
    pub limit: u64,
    pub offset: u64,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, count: u64, limit: Option<u64>, offset: Option<u64>) -> Self {
        Self {
            items,
            count,
            limit: limit.unwrap_or(count),
            offset: offset.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct Pagination {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

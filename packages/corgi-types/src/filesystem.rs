use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FileEntry {
    #[schema(example = "dir")]
    pub name: String,

    #[schema(example = "/path/to/dir")]
    pub path: String,

    #[schema(example = "true")]
    pub is_dir: bool,

    #[schema(example = "false")]
    pub is_file: bool,

    #[schema(example = "false")]
    pub is_symlink: bool,
}

use std::path::Path;

use tokio::fs::read_dir;

use corgi_types::FileEntry;

pub async fn ls<P: AsRef<Path>>(root: Option<P>, only_dir: bool) -> Vec<FileEntry> {
    let root = root
        .as_ref()
        .map(|p| p.as_ref())
        .unwrap_or(Path::new("/"))
        .to_path_buf();

    let mut entries = vec![];

    let mut dir = match read_dir(&root).await {
        Ok(dir) => dir,
        Err(e) => {
            tracing::error!("Failed to read directory: {:?}", e);
            return entries;
        }
    };

    while let Ok(Some(entry)) = dir.next_entry().await {
        let path = entry.path();

        if super::is_hidden(&entry.file_name()) {
            continue;
        }

        if only_dir && !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();

        entries.push(FileEntry {
            name,
            path: path.to_string_lossy().into_owned(),
            is_dir: path.is_dir(),
            is_file: path.is_file(),
            is_symlink: path.is_symlink(),
        });
    }

    entries
}

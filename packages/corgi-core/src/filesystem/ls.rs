use std::path::Path;

use walkdir::WalkDir;

use corgi_types::FileEntry;

pub fn ls<P: AsRef<Path>>(root: Option<P>, only_dir: bool) -> Vec<FileEntry> {
    let root = root.as_ref().map(|p| p.as_ref()).unwrap_or(Path::new("/"));

    let walker = WalkDir::new(root)
        .max_depth(1)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter();

    let entries = walker
        .filter_entry(|e| !super::is_hidden(e) && (!only_dir || e.path().is_dir()))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            Some(FileEntry {
                name,
                path: path.to_string_lossy().into_owned(),
                is_dir: path.is_dir(),
                is_file: path.is_file(),
                is_symlink: path.is_symlink(),
            })
        });

    entries.collect()
}

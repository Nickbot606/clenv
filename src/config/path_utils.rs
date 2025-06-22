use std::env;
use std::path::{Path, PathBuf};

// Small helper function to resolve strings, relative paths, and absolute paths
pub fn resolve_path(pathin: &str, file_ext: &str) -> PathBuf {
    let path = Path::new(pathin);

    if path.is_absolute() {
        path.to_path_buf()
    } else if path.components().count() > 0 {
        env::current_dir()
            .unwrap()
            .join(path)
            .canonicalize()
            .unwrap_or_else(|_| {
                let mut joined = env::current_dir().unwrap().join(path);
                if !joined.extension().is_some() && !file_ext.is_empty() {
                    joined.set_extension(file_ext);
                }
                joined
            })
    } else {
        let mut new_path = env::current_dir().unwrap();
        new_path.push(pathin);
        if !file_ext.is_empty() {
            new_path.set_extension(file_ext);
        }
        new_path
    }
}
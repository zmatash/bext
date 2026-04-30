use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

fn search_dir_for(dir: &Path, file_name: &str) -> Option<PathBuf> {
    for entry in read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.file_name()?.to_str()? == file_name {
            return Some(path);
        }
    }
    None
}

pub fn search_up_for_file<P: AsRef<Path>>(start: P, file_name: &str) -> Option<PathBuf> {
    let mut current_dir = start.as_ref().to_path_buf();

    loop {
        if let Some(path) = search_dir_for(&current_dir, file_name) {
            return Some(path);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

pub fn search_down_for_file<P: AsRef<Path>>(start: P, file_name: &str) -> Option<PathBuf> {
    WalkDir::new(start)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == file_name)
        .map(|e| e.into_path())
}

#[cfg(test)]
mod tests {
    use crate::ops::find_files::{search_down_for_file, search_up_for_file};

    #[test]
    fn test_find_file_in_tree() {
        const NESTED_COUNT: usize = 5;
        const FILE_NAME: &str = "target.txt";

        let tempdir = tempfile::tempdir().unwrap();

        let mut current_dir = tempdir.path().to_path_buf();
        for i in 0..NESTED_COUNT {
            current_dir.push(format!("nested_{}", i));
            std::fs::create_dir(&current_dir).unwrap();
        }

        let target_file = current_dir.join(FILE_NAME);
        std::fs::write(&target_file, "test").unwrap();

        let file = search_up_for_file(&current_dir, FILE_NAME).unwrap();
        assert_eq!(file, target_file);
    }

    #[test]
    fn test_search_down_finds_nested_file() {
        let tempdir = tempfile::tempdir().unwrap();
        let nested = tempdir.path().join("a").join("b");
        std::fs::create_dir_all(&nested).unwrap();
        let target = nested.join("target.txt");
        std::fs::write(&target, "test").unwrap();

        let found = search_down_for_file(tempdir.path(), "target.txt").unwrap();
        assert_eq!(found, target);
    }

    #[test]
    fn test_search_down_returns_none_when_missing() {
        let tempdir = tempfile::tempdir().unwrap();
        assert!(search_down_for_file(tempdir.path(), "ghost.txt").is_none());
    }
}

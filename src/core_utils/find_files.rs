use std::{fs::read_dir, path::PathBuf};

fn search_dir_for(dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
    for entry in read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() && path.file_name()?.to_str()? == file_name {
            return Some(path);
        }
    }
    None
}

pub fn search_up_for_file(file_name: &str) -> Option<PathBuf> {
    let mut current_dir = std::env::current_dir().ok()?;

    loop {
        if let Some(path) = search_dir_for(&current_dir, file_name) {
            return Some(path);
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core_utils::find_files::search_up_for_file;

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

        std::env::set_current_dir(&current_dir).unwrap();
        let file = search_up_for_file(FILE_NAME).unwrap();
        assert_eq!(file, target_file);
    }
}

use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Path exists: {0}")]
    PathExists(PathBuf),

    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),
}

#[cfg(windows)]
fn create_junction<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    junction::create(source, link)?;
    Ok(())
}

#[cfg(unix)]
fn create_junction<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    std::os::unix::fs::symlink(source, link)?;
    Ok(())
}

pub fn create_link<P: AsRef<Path>, Q: AsRef<Path>>(
    source: P,
    link: Q,
    replace_existing: bool,
) -> Result<(), LinkError> {
    if link.as_ref().exists() {
        if replace_existing {
            if link.as_ref().is_dir() {
                std::fs::remove_dir_all(&link)?;
            } else {
                std::fs::remove_file(&link)?;
            }
        } else {
            return Err(LinkError::PathExists(link.as_ref().to_path_buf()));
        }
    }

    create_junction(source, link)
}

pub fn remove_link<P: AsRef<Path>>(link: P) -> Result<(), LinkError> {
    let link = link.as_ref();

    if !link.exists() {
        return Err(LinkError::PathNotFound(link.to_path_buf()));
    }

    if link.is_dir() {
        std::fs::remove_dir_all(link)?;
    } else {
        std::fs::remove_file(link)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_create_junction() {
        let temp_dir = std::env::temp_dir();
        let source_dir = temp_dir.join("test_source");
        let link_dir = temp_dir.join("test_link");

        fs::create_dir_all(&source_dir).unwrap();
        let mut test_file = File::create(source_dir.join("test.txt")).unwrap();
        writeln!(test_file, "Hello, world!").unwrap();

        super::create_link(&source_dir, &link_dir, false).unwrap();

        assert!(link_dir.exists());
        assert!(link_dir.join("test.txt").exists());

        fs::remove_file(source_dir.join("test.txt")).unwrap();
        fs::remove_dir_all(&source_dir).unwrap();
        fs::remove_dir_all(&link_dir).unwrap();
    }

    #[test]
    fn test_create_junction_replace() {
        let temp_dir = std::env::temp_dir();
        let source_dir1 = temp_dir.join("test_source_replace1");
        let source_dir2 = temp_dir.join("test_source_replace2");
        let link_dir = temp_dir.join("test_link_replace");

        fs::create_dir_all(&source_dir1).unwrap();
        fs::create_dir_all(&source_dir2).unwrap();
        File::create(source_dir1.join("file1.txt")).unwrap();
        File::create(source_dir2.join("file2.txt")).unwrap();

        super::create_link(&source_dir1, &link_dir, false).unwrap();
        assert!(link_dir.join("file1.txt").exists());

        super::create_link(&source_dir2, &link_dir, true).unwrap();
        assert!(link_dir.exists());
        assert!(link_dir.join("file2.txt").exists());
        assert!(!link_dir.join("file1.txt").exists());

        fs::remove_dir_all(&source_dir1).unwrap();
        fs::remove_dir_all(&source_dir2).unwrap();
        fs::remove_dir_all(&link_dir).unwrap();
    }

    #[test]
    fn test_remove_link() {
        let temp_dir = std::env::temp_dir();
        let source_dir = temp_dir.join("test_source_remove");
        let link_dir = temp_dir.join("test_link_remove");

        fs::create_dir_all(&source_dir).unwrap();
        super::create_link(&source_dir, &link_dir, false).unwrap();
        assert!(link_dir.exists());

        super::remove_link(&link_dir).unwrap();
        assert!(!link_dir.exists());

        fs::remove_dir_all(&source_dir).unwrap();
    }
}

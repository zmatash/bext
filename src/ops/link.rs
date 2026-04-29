use std::{path::Path, process::Command};

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unsupported platform")]
    UnsupportedPlatform,

    #[error("Path contains invalid UTF-8: {0}")]
    InvalidPath(std::path::PathBuf),

    #[error("Failed to create Junction via cmd")]
    JunctionCreationFailed,

    #[error("Failed to remove Junction via cmd")]
    JunctionRemovalFailed,
}

#[cfg(windows)]
fn create_junction<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    let source = source.as_ref();
    let link = link.as_ref();
    let source_str = source
        .to_str()
        .ok_or_else(|| LinkError::InvalidPath(source.to_path_buf()))?;
    let link_str = link
        .to_str()
        .ok_or_else(|| LinkError::InvalidPath(link.to_path_buf()))?;

    let status = Command::new("cmd")
        .args(["/C", "mklink", "/J", link_str, source_str])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(LinkError::JunctionCreationFailed)
    }
}

#[cfg(windows)]
fn remove_junction<P: AsRef<Path>>(link: P) -> Result<(), LinkError> {
    let link = link.as_ref();
    let link_str = link
        .to_str()
        .ok_or_else(|| LinkError::InvalidPath(link.to_path_buf()))?;

    let status = Command::new("cmd")
        .args(["/C", "rmdir", link_str])
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(LinkError::JunctionRemovalFailed)
    }
}

#[cfg(unix)]
fn remove_junction<P: AsRef<Path>>(link: P) -> Result<(), LinkError> {
    std::fs::remove_file(link)?;
    Ok(())
}

#[cfg(unix)]
fn create_junction<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    std::os::unix::fs::symlink(source, link)?;
    Ok(())
}

pub fn create_link<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    create_junction(source, link)
}

pub fn remove_link<P: AsRef<Path>>(link: P) -> Result<(), LinkError> {
    remove_junction(link)
}

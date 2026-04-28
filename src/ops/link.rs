use std::{path::Path, process::Command};

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unsupported platform")]
    UnsupportedPlatform,

    #[error("Failed to create Junction via cmd")]
    JunctionCreationFailed,

    #[error("Failed to remove Junction via cmd")]
    JunctionRemovalFailed,
}

#[cfg(windows)]
fn create_junction<P: AsRef<Path>, Q: AsRef<Path>>(source: P, link: Q) -> Result<(), LinkError> {
    let source_str = source.as_ref().to_str().expect("Invalid source path");
    let link_str = link.as_ref().to_str().expect("Invalid link path");

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
    let link_str = link.as_ref().to_str().expect("Invalid link path");

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

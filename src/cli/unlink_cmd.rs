use std::{env::current_dir, path::PathBuf};

use crate::{
    manifests::bext_config::BextConfig,
    ops::{blender_data, link_ops},
};

#[derive(Debug, thiserror::Error)]
pub enum UnlinkCommandError {
    #[error("Unable to determine current directory")]
    CurrentDirError(#[from] std::io::Error),

    #[error("Bext configuration error: {0}")]
    BextConfigError(#[from] crate::manifests::bext_config::BextConfigError),

    #[error("No Blender versions specified in configuration")]
    NoBlenderVersions,

    #[error("Blender data error: {0}")]
    BlenderDataError(#[from] blender_data::BlenderDataError),

    #[error("Invalid source path: {0}")]
    InvalidSourcePath(String),

    #[error("Link error: {0}")]
    LinkError(#[from] link_ops::LinkError),
}

pub struct UnlinkResult {
    pub removed: Vec<PathBuf>,
    pub not_found: Vec<PathBuf>,
}

pub fn run_unlink_command() -> Result<UnlinkResult, UnlinkCommandError> {
    let current_dir = current_dir()?;
    let config = BextConfig::from_config_search(&current_dir)?;
    let versions = match config.blender_versions {
        Some(vs) if !vs.is_empty() => vs,
        _ => return Err(UnlinkCommandError::NoBlenderVersions),
    };

    let source_path = current_dir.join(&config.source_dir);
    let dir_name = match source_path.file_name() {
        Some(name) => name,
        None => {
            return Err(UnlinkCommandError::InvalidSourcePath(
                source_path.to_string_lossy().to_string(),
            ));
        }
    };

    let mut removed: Vec<PathBuf> = Vec::new();
    let mut not_found: Vec<PathBuf> = Vec::new();

    for version in versions {
        let ext_dir = blender_data::get_blender_extension_dir(version)?;
        let target_path = ext_dir.join(dir_name);
        match link_ops::remove_link(&target_path) {
            Ok(()) => removed.push(target_path),
            Err(link_ops::LinkError::PathNotFound(_)) => not_found.push(target_path),
            Err(e) => return Err(UnlinkCommandError::LinkError(e)),
        }
    }

    Ok(UnlinkResult { removed, not_found })
}

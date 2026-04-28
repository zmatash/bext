use std::env::current_dir;

use crate::{
    core_utils::{blender_data, link},
    manifests::bext_config::BextConfig,
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
    LinkError(#[from] link::LinkError),
}

pub fn run_unlink_command() -> Result<(), UnlinkCommandError> {
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

    for version in versions {
        let ext_dir = blender_data::get_blender_extension_dir(version)?;
        let target_path = ext_dir.join(dir_name);
        link::remove_link(&target_path)?;
    }

    Ok(())
}

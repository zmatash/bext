use std::{env::current_dir, path::PathBuf};

use crate::{
    manifests::bext_config::BextConfig,
    ops::{
        blender_data,
        extension_validation::{self, validate_extension},
        link_ops::{self, LinkError},
    },
};

#[derive(Debug, thiserror::Error)]
pub enum LinkCommandError {
    #[error("Unable to determine current directory")]
    CurrentDirError(#[from] std::io::Error),

    #[error("Bext configuration error: {0}")]
    BextConfigError(#[from] crate::manifests::bext_config::BextConfigError),

    #[error("No Blender versions specified in configuration")]
    NoBlenderVersions,

    #[error("Blender data error: {0}")]
    BlenderDataError(#[from] blender_data::BlenderDataError),

    #[error("Extension validation error: {0}")]
    ExtensionValidationError(#[from] extension_validation::ExtensionValidationError),

    #[error("Source path is not valid: {0}")]
    InvalidSourcePath(String),

    #[error("Link error: {0}")]
    LinkError(#[from] link_ops::LinkError),
}

pub struct LinkResult {
    pub linked: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
}

pub fn run_link_command(replace_existing: bool) -> Result<LinkResult, LinkCommandError> {
    let current_dir = current_dir()?;
    let config = BextConfig::from_config_search(&current_dir)?;
    let versions = match config.blender_versions {
        Some(vs) if !vs.is_empty() => vs,
        _ => return Err(LinkCommandError::NoBlenderVersions),
    };

    let source_path = current_dir.join(&config.source_dir);
    validate_extension(&source_path)?;

    let mut linked: Vec<PathBuf> = Vec::new();
    let mut skipped: Vec<PathBuf> = Vec::new();

    for version in versions {
        let ext_dir = blender_data::get_blender_extension_dir(version)?;
        let dir_name = match source_path.file_name() {
            Some(name) => name,
            None => {
                return Err(LinkCommandError::InvalidSourcePath(
                    source_path.to_string_lossy().to_string(),
                ));
            }
        };

        let target_path = ext_dir.join(dir_name);
        match link_ops::create_link(&source_path, &target_path, replace_existing) {
            Ok(()) => linked.push(target_path),
            Err(LinkError::PathExists(_)) => skipped.push(target_path),
            Err(e) => return Err(LinkCommandError::LinkError(e)),
        }
    }

    Ok(LinkResult { linked, skipped })
}

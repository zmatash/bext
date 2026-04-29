use std::{env::current_dir, fs};

use crate::{
    manifests::{
        bext_config::{self, BextConfig},
        blender_manifest::{self, BlenderManifest},
    },
    ops::{
        archive_ops,
        extension_validation::{self, validate_extension},
        glob_ops,
    },
};

#[derive(Debug, thiserror::Error)]
pub enum BuildCommandError {
    #[error("Archive error: {0}")]
    ArchiveError(#[from] archive_ops::ArchiveOpsError),
    #[error("Glob error: {0}")]
    GlobError(#[from] glob_ops::GlobDeleteError),
    #[error("Unable to determine current directory")]
    CurrentDirError(#[from] std::io::Error),
    #[error("Bext configuration error: {0}")]
    BextConfigError(#[from] bext_config::BextConfigError),
    #[error("No output directory specified in configuration")]
    NoOutputDirectory,
    #[error("Blender manifest error: {0}")]
    BlenderManifestError(#[from] blender_manifest::BlenderManifestError),
    #[error("Source path does not exist or is not a directory: {0}")]
    InvalidSourcePath(std::path::PathBuf),
    #[error("Output path exists but is not a directory: {0}")]
    InvalidOutputPath(std::path::PathBuf),
    #[error("Extension validation error: {0}")]
    ExtensionValidationError(#[from] extension_validation::ExtensionValidationError),
}

pub fn run_build_command() -> Result<(), BuildCommandError> {
    let current_dir = current_dir()?;
    let config = BextConfig::from_config_search(&current_dir)?;
    let src_dir = current_dir.join(&config.source_dir);
    if !src_dir.exists() || !src_dir.is_dir() {
        return Err(BuildCommandError::InvalidSourcePath(src_dir));
    }

    validate_extension(&src_dir)?;
    let extension_manifest = BlenderManifest::from_file(src_dir.join("blender_manifest.toml"))?;

    let out_dir = match config.output_dir {
        Some(ref dir) if !dir.is_empty() => {
            let full_out_path = current_dir.join(dir);
            if !full_out_path.exists() {
                fs::create_dir_all(&full_out_path)?;
            } else if !full_out_path.is_dir() {
                return Err(BuildCommandError::InvalidOutputPath(full_out_path));
            }
            full_out_path
        }
        _ => return Err(BuildCommandError::NoOutputDirectory),
    };

    let dst_file = out_dir.join(format!(
        "{}.zip",
        config.resolve_package_name(&extension_manifest)
    ));

    let exclude_globs: Vec<glob::Pattern> = match &config.exclude_globs {
        Some(patterns) if !patterns.is_empty() => glob_ops::compile_string_globs(patterns)?,
        _ => Vec::new(),
    };

    archive_ops::build_archive(src_dir, dst_file, &exclude_globs)?;
    Ok(())
}

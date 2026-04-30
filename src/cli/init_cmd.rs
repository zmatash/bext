use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::{
    manifests::{
        bext_config::{self, BextConfig},
        blender_manifest::{self},
    },
    ops::find_files::search_down_for_file,
};

#[derive(Debug, thiserror::Error)]
pub enum InitCommandError {
    #[error("Unable to determine current directory")]
    CurrentDirError(#[from] std::io::Error),
    #[error("Bext configuration already exists at {0}")]
    ConfigAlreadyExists(PathBuf),
    #[error("Extension manifest not found in current directory or any subdirectories")]
    ManifestNotFound,
    #[error("Blender manifest error: {0}")]
    BlenderManifestError(#[from] blender_manifest::BlenderManifestError),
    #[error("Bext configuration error: {0}")]
    BextConfigError(#[from] bext_config::BextConfigError),
}

pub fn run_init_command() -> Result<(), InitCommandError> {
    let current_dir = current_dir()?;
    let config_path = current_dir.join("bext.toml");
    if config_path.exists() {
        return Err(InitCommandError::ConfigAlreadyExists(config_path));
    }

    match search_down_for_file(&current_dir, "blender_manifest.toml") {
        Some(manifest_path) => {
            let source_dir = manifest_path
                .parent()
                .and_then(|p| p.strip_prefix(&current_dir).ok())
                .unwrap_or(Path::new("."));
            BextConfig::builder(source_dir.to_string_lossy())
                .exclude_globs(vec!["**/__pycache__/**".into(), "**/*.pyc".into()])
                .blender_versions(vec![])
                .output_dir("dist")
                .package_name("{name} ({version})")
                .build()
                .to_file(config_path)?;
            Ok(())
        }

        None => Err(InitCommandError::ManifestNotFound),
    }
}

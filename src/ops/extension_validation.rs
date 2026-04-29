use std::path::{Path, PathBuf};

// TODO: Add validation of the manifest file contents.

#[derive(Debug, thiserror::Error)]
pub enum ExtensionValidationError {
    #[error("Directory does not exist: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("No blender_extension.toml file found in source directory {0}")]
    NoManifestFile(PathBuf),
}

pub fn validate_extension<P: AsRef<Path>>(source_dir: P) -> Result<(), ExtensionValidationError> {
    let source_dir = source_dir.as_ref();
    if !source_dir.exists() {
        return Err(ExtensionValidationError::DirectoryNotFound(
            source_dir.into(),
        ));
    }

    let manifest_path = source_dir.join("blender_manifest.toml");
    if !manifest_path.exists() {
        return Err(ExtensionValidationError::NoManifestFile(source_dir.into()));
    }

    Ok(())
}

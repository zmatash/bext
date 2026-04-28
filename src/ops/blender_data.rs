use std::path::PathBuf;

use semver::Version;

#[derive(Debug, thiserror::Error)]
pub enum BlenderDataError {
    #[error("Unable to determine data directory")]
    DataDirNotFound,
    #[error("Blender data directory not found: {0}")]
    BlenderDataDirNotFound(PathBuf),
    #[error("Blender data directory not found for version {0}")]
    BlenderVersionNotFound(Version),
    #[error("Blender extensions directory not found: {0}")]
    BlenderExtensionsDirNotFound(PathBuf),
}

pub fn get_blender_extension_dir(version: Version) -> Result<PathBuf, BlenderDataError> {
    let data_dir = dirs::data_dir().ok_or(BlenderDataError::DataDirNotFound)?;

    let version_string = format!("{}.{}", version.major, version.minor);
    let blender_dir = data_dir.join("Blender Foundation").join("Blender");

    if !blender_dir.exists() {
        return Err(BlenderDataError::BlenderDataDirNotFound(blender_dir));
    }

    let version_dir = blender_dir.join(&version_string);

    if !version_dir.exists() {
        return Err(BlenderDataError::BlenderVersionNotFound(version));
    }

    let path = version_dir.join("extensions").join("user_default");

    if !path.exists() {
        return Err(BlenderDataError::BlenderExtensionsDirNotFound(path));
    }

    Ok(path)
}

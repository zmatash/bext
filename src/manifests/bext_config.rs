use std::path::PathBuf;

use semver::Version;
use serde::Deserialize;
use thiserror::Error;
use toml_edit::DocumentMut;

use crate::manifests::utility::set_optional;

#[derive(Error, Debug)]
pub enum BextConfigError {
    #[error("TOML deserialization error: {0}")]
    DeserializeError(#[from] toml_edit::de::Error),
    #[error("TOML parse error: {0}")]
    ParseError(#[from] toml_edit::TomlError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BextConfig {
    pub exclude_globs: Option<Vec<String>>,
    pub blender_versions: Option<Vec<Version>>,
    pub source_dir: String,
    pub output_dir: String,

    #[serde(skip)]
    doc: DocumentMut,
}

impl std::str::FromStr for BextConfig {
    type Err = BextConfigError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut pyproject: BextConfig = toml_edit::de::from_str(content)?;
        pyproject.doc = content.parse()?;
        Ok(pyproject)
    }
}

impl BextConfig {
    pub fn from_file(path: &PathBuf) -> Result<Self, BextConfigError> {
        let content = std::fs::read_to_string(path)?;
        content.parse()
    }

    pub fn to_string(&self) -> Result<String, BextConfigError> {
        let mut doc = self.doc.clone();
        set_optional(&mut doc, "exclude_globs", self.exclude_globs.as_ref());
        set_optional(&mut doc, "blender_versions", self.blender_versions.as_ref());
        doc["source_dir"] = toml_edit::value(&self.source_dir);
        doc["output_dir"] = toml_edit::value(&self.output_dir);

        Ok(doc.to_string())
    }

    pub fn to_file(&self, path: &PathBuf) -> Result<(), BextConfigError> {
        let content = self.to_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

use std::path::PathBuf;

use pep440_rs::Version;
use serde::Deserialize;
use thiserror::Error;
use toml_edit::DocumentMut;

#[derive(Error, Debug)]
pub enum PyProjectError {
    #[error("invalid TOML: {0}")]
    Toml(#[from] toml_edit::TomlError),
    #[error("deserialize: {0}")]
    De(#[from] toml_edit::de::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub name: String,
    pub version: Version,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PyProject {
    pub project: Project,

    #[serde(skip)]
    doc: DocumentMut,
}

impl std::str::FromStr for PyProject {
    type Err = PyProjectError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut pyproject: PyProject = toml_edit::de::from_str(content)?;
        pyproject.doc = content.parse()?;
        Ok(pyproject)
    }
}

impl PyProject {
    pub fn from_file(path: &PathBuf) -> Result<Self, PyProjectError> {
        let content = std::fs::read_to_string(path)?;
        content.parse()
    }

    pub fn to_string(&self) -> Result<String, PyProjectError> {
        let mut doc = self.doc.clone();
        doc["project"]["name"] = toml_edit::value(&self.project.name);
        doc["project"]["version"] = toml_edit::value(self.project.version.to_string());
        Ok(doc.to_string())
    }

    pub fn to_file(&self, path: &PathBuf) -> Result<(), PyProjectError> {
        let content = self.to_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

use std::path::PathBuf;

use semver::Version;
use serde::Deserialize;
use thiserror::Error;
use toml_edit::DocumentMut;

use crate::manifests::common::{ToInlineTable, set_optional};

#[derive(Error, Debug)]
pub enum BlenderManifestError {
    #[error("TOML deserialization error: {0}")]
    DeserializeError(#[from] toml_edit::de::Error),

    #[error("TOML parse error: {0}")]
    ParseError(#[from] toml_edit::TomlError),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Permissions {
    pub files: Option<String>,
    pub network: Option<String>,
    pub clipboard: Option<String>,
    pub camera: Option<String>,
    pub microphone: Option<String>,
}

impl ToInlineTable for Permissions {
    fn to_inline_table(&self) -> toml_edit::InlineTable {
        let mut tbl = toml_edit::InlineTable::new();
        if let Some(v) = &self.files {
            tbl.insert("files", v.into());
        }
        if let Some(v) = &self.network {
            tbl.insert("network", v.into());
        }
        if let Some(v) = &self.clipboard {
            tbl.insert("clipboard", v.into());
        }
        if let Some(v) = &self.camera {
            tbl.insert("camera", v.into());
        }
        if let Some(v) = &self.microphone {
            tbl.insert("microphone", v.into());
        }
        tbl
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ExtensionType {
    #[serde(rename = "add-on")]
    AddOn,
    #[serde(rename = "theme")]
    Theme,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlenderManifest {
    pub id: String,
    pub version: Version,
    pub name: String,
    pub maintainer: String,
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,

    pub website: Option<String>,
    pub tags: Option<Vec<String>>,
    pub blender_version_min: Option<Version>,
    pub blender_version_max: Option<Version>,
    pub wheels: Option<Vec<String>>,
    pub permissions: Option<Permissions>,

    #[serde(skip)]
    doc: DocumentMut,
}

impl std::str::FromStr for BlenderManifest {
    type Err = BlenderManifestError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut pyproject: BlenderManifest = toml_edit::de::from_str(content)?;
        pyproject.doc = content.parse()?;
        Ok(pyproject)
    }
}

impl BlenderManifest {
    pub fn from_file(path: &PathBuf) -> Result<Self, BlenderManifestError> {
        let content = std::fs::read_to_string(path)?;
        content.parse()
    }

    pub fn to_string(&self) -> Result<String, BlenderManifestError> {
        let mut doc = self.doc.clone();

        doc["id"] = toml_edit::value(&self.id);
        doc["version"] = toml_edit::value(self.version.to_string());
        doc["name"] = toml_edit::value(&self.name);
        doc["maintainer"] = toml_edit::value(&self.maintainer);
        doc["type"] = toml_edit::value(match self.extension_type {
            ExtensionType::AddOn => "add-on",
            ExtensionType::Theme => "theme",
        });

        set_optional(&mut doc, "website", self.website.as_deref());
        set_optional(&mut doc, "tags", self.tags.as_ref());
        set_optional(
            &mut doc,
            "blender_version_min",
            self.blender_version_min.as_ref(),
        );
        set_optional(
            &mut doc,
            "blender_version_max",
            self.blender_version_max.as_ref(),
        );
        set_optional(&mut doc, "wheels", self.wheels.as_ref());

        set_optional(
            &mut doc,
            "permissions",
            self.permissions.as_ref().map(|p| p.to_inline_table()),
        );

        Ok(doc.to_string())
    }

    pub fn to_file(&self, path: &PathBuf) -> Result<(), BlenderManifestError> {
        let content = self.to_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

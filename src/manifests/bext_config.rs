use std::path::Path;

use semver::Version;
use serde::{Deserialize, Deserializer};
use thiserror::Error;
use toml_edit::DocumentMut;

use crate::{
    manifests::{blender_manifest::BlenderManifest, common::set_optional},
    ops::find_files,
};

fn deserialize_lenient_versions<'de, D>(deserializer: D) -> Result<Option<Vec<Version>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<Vec<String>> = Option::deserialize(deserializer)?;
    let Some(strings) = opt else { return Ok(None) };

    let versions = strings
        .into_iter()
        .map(|s| {
            Version::parse(&s)
                .or_else(|_| Version::parse(&format!("{}.0", s)))
                .or_else(|_| Version::parse(&format!("{}.0.0", s)))
                .map_err(serde::de::Error::custom)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some(versions))
}

#[derive(Error, Debug)]
pub enum BextConfigError {
    #[error("TOML deserialization error: {0}")]
    DeserializeError(#[from] toml_edit::de::Error),
    #[error("TOML parse error: {0}")]
    ParseError(#[from] toml_edit::TomlError),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Config file not found in directory tree starting from: {0}")]
    ConfigNotFound(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct BextConfig {
    pub exclude_globs: Option<Vec<String>>,
    #[serde(default, deserialize_with = "deserialize_lenient_versions")]
    pub blender_versions: Option<Vec<Version>>,
    pub source_dir: String,
    pub output_dir: Option<String>,
    pub package_name: Option<String>,

    #[serde(skip)]
    doc: DocumentMut,
}

impl std::str::FromStr for BextConfig {
    type Err = BextConfigError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut bext_config: BextConfig = toml_edit::de::from_str(content)?;
        bext_config.doc = content.parse()?;
        Ok(bext_config)
    }
}

impl BextConfig {
    pub fn from_config_search<P: AsRef<Path>>(search_start: P) -> Result<BextConfig, BextConfigError> {
        let search_start = search_start.as_ref();
        match find_files::search_up_for_file(search_start, "bext.toml") {
            Some(path) => Self::from_file(&path),
            None => Err(BextConfigError::ConfigNotFound(
                search_start.to_string_lossy().to_string(),
            )),
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, BextConfigError> {
        let content = std::fs::read_to_string(path)?;
        content.parse()
    }

    pub fn to_string(&self) -> Result<String, BextConfigError> {
        let mut doc = self.doc.clone();
        set_optional(&mut doc, "exclude_globs", self.exclude_globs.as_ref());
        set_optional(&mut doc, "blender_versions", self.blender_versions.as_ref());
        set_optional(&mut doc, "output_dir", self.output_dir.as_ref());
        set_optional(&mut doc, "package_name", self.package_name.as_ref());
        doc["source_dir"] = toml_edit::value(&self.source_dir);

        Ok(doc.to_string())
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), BextConfigError> {
        let content = self.to_string()?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn resolve_package_name(&self, blender_manifest: &BlenderManifest) -> String {
        let template = self.package_name.as_deref().unwrap_or("{name}");
        template
            .replace("{id}", &blender_manifest.id)
            .replace("{name}", &blender_manifest.name)
            .replace("{version}", &blender_manifest.version.to_string())
            .replace("{maintainer}", &blender_manifest.maintainer)
    }
}

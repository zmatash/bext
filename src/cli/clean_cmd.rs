use std::env::current_dir;

use crate::{
    manifests::bext_config::{self, BextConfig},
    ops::{
        extension_validation::{self, validate_extension},
        glob_ops,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum CleanCommandError {
    #[error("Unable to determine current directory")]
    CurrentDirError(#[from] std::io::Error),

    #[error("Bext configuration error: {0}")]
    BextConfigError(#[from] bext_config::BextConfigError),

    #[error("No exclude patterns specified in configuration")]
    NoExcludePatterns,

    #[error("Extension validation error: {0}")]
    ExtensionValidationError(#[from] extension_validation::ExtensionValidationError),

    #[error("Glob delete error: {0}")]
    GlobDeleteError(#[from] glob_ops::GlobDeleteError),
}

pub fn run_clean_command() -> Result<(), CleanCommandError> {
    let current_dir = current_dir()?;
    let config = BextConfig::from_config_search(&current_dir)?;

    let exclude_glob = match &config.exclude_globs {
        Some(patterns) if !patterns.is_empty() => glob_ops::compile_string_globs(patterns)?,
        _ => return Err(CleanCommandError::NoExcludePatterns),
    };

    let source_path = current_dir.join(&config.source_dir);
    validate_extension(&source_path)?;

    let deleted = glob_ops::glob_delete(&source_path, &exclude_glob)?;
    println!("Deleted {deleted} items based on exclude patterns.");

    Ok(())
}

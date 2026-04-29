use std::{fs, path::Path};

use glob::Pattern;
use walkdir::WalkDir;

#[derive(Debug, thiserror::Error)]
pub enum GlobDeleteError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Glob pattern error: {0}")]
    GlobError(#[from] glob::PatternError),
}

pub fn compile_string_globs<T: AsRef<str>>(globs: &[T]) -> Result<Vec<Pattern>, GlobDeleteError> {
    let compiled_globs: Vec<Pattern> = globs
        .iter()
        .filter_map(|p| Pattern::new(p.as_ref()).ok())
        .collect();
    Ok(compiled_globs)
}
pub fn glob_delete(dir: &Path, globs: &[Pattern]) -> Result<(), GlobDeleteError> {
    for entry in WalkDir::new(dir)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let should_delete = globs.iter().any(|p| p.matches(file_name));

        if should_delete {
            if path.is_file() {
                fs::remove_file(path)?;
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
        }
    }

    Ok(())
}

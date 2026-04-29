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
    globs
        .iter()
        .map(|p| Pattern::new(p.as_ref()).map_err(GlobDeleteError::from))
        .collect()
}

pub fn glob_delete(dir: &Path, globs: &[Pattern]) -> Result<u32, GlobDeleteError> {
    let mut items_deleted: u32 = 0;

    for entry in WalkDir::new(dir)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let Ok(rel_path) = path.strip_prefix(dir) else {
            continue;
        };

        if rel_path.as_os_str().is_empty() {
            continue;
        }

        let should_delete = globs.iter().any(|p| p.matches_path(rel_path));

        if should_delete {
            items_deleted += 1;
            if path.is_file() {
                fs::remove_file(path)?;
            } else if path.is_dir() {
                fs::remove_dir_all(path)?;
            }
        }
    }

    Ok(items_deleted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_string_globs_propagates_errors() {
        let result = compile_string_globs(&["valid/**", "[invalid"]);
        assert!(matches!(result, Err(GlobDeleteError::GlobError(_))));
    }

    #[test]
    fn test_glob_delete_matches_nested_paths() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path();

        let nested_dir = root.join("pkg").join("__pycache__");
        fs::create_dir_all(&nested_dir).unwrap();
        fs::write(nested_dir.join("mod.cpython-311.pyc"), "x").unwrap();
        let kept = root.join("pkg").join("keep.py");
        fs::write(&kept, "x").unwrap();

        let globs = compile_string_globs(&["**/__pycache__/**", "**/__pycache__"]).unwrap();
        glob_delete(root, &globs).unwrap();

        assert!(!nested_dir.exists());
        assert!(kept.exists());
    }

    #[test]
    fn test_glob_delete_matches_extension_pattern() {
        let tempdir = tempfile::tempdir().unwrap();
        let root = tempdir.path();

        let pyc = root.join("a").join("b.pyc");
        fs::create_dir_all(pyc.parent().unwrap()).unwrap();
        fs::write(&pyc, "x").unwrap();
        let py = root.join("a").join("b.py");
        fs::write(&py, "x").unwrap();

        let globs = compile_string_globs(&["**/*.pyc"]).unwrap();
        glob_delete(root, &globs).unwrap();

        assert!(!pyc.exists());
        assert!(py.exists());
    }
}

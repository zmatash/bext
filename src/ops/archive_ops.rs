use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Seek, Write},
    path::{Path, PathBuf},
};

use glob::Pattern;
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

#[derive(Debug, thiserror::Error)]
pub enum ArchiveOpsError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WalkDir error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Path error: {0}")]
    PathError(#[from] std::path::StripPrefixError),
}

pub fn build_archive<P: AsRef<Path>>(
    src_dir: P,
    dst_file: P,
    exclude_globs: &[Pattern],
) -> Result<(), ArchiveOpsError> {
    let src_dir = src_dir.as_ref();

    let file = File::create(dst_file)?;
    let mut zip_writer = ZipWriter::new(BufWriter::new(file));

    let options = FileOptions::<()>::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walker = WalkDir::new(src_dir).into_iter().filter_entry(|e| {
        let Ok(rel_path) = e.path().strip_prefix(src_dir) else {
            return true;
        };

        if rel_path.as_os_str().is_empty() {
            return true;
        }

        !exclude_globs
            .iter()
            .any(|pattern| pattern.matches_path(rel_path))
    });

    // Directory entries are written lazily so that directories left empty by
    // exclusions (e.g. `__pycache__` when only `**/__pycache__/**` is excluded)
    // are not added to the archive. Directories that are empty in the source
    // are still included.
    let mut written_dirs: HashSet<PathBuf> = HashSet::new();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        let name = path.strip_prefix(src_dir)?;
        if name.as_os_str().is_empty() {
            continue;
        }

        if path.is_file() {
            if let Some(parent) = name.parent() {
                write_dir_entries(&mut zip_writer, &mut written_dirs, parent, options)?;
            }
            zip_writer.start_file(zip_name(name), options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip_writer)?;
        } else if path.is_dir() && std::fs::read_dir(path)?.next().is_none() {
            write_dir_entries(&mut zip_writer, &mut written_dirs, name, options)?;
        }
    }

    zip_writer.finish()?;
    Ok(())
}

fn zip_name(rel_path: &Path) -> String {
    rel_path.to_string_lossy().replace('\\', "/")
}

/// Writes directory entries for `rel_dir` and any of its ancestors not yet written.
fn write_dir_entries<W: Write + Seek>(
    zip_writer: &mut ZipWriter<W>,
    written_dirs: &mut HashSet<PathBuf>,
    rel_dir: &Path,
    options: FileOptions<()>,
) -> Result<(), ArchiveOpsError> {
    let mut pending: Vec<&Path> = rel_dir
        .ancestors()
        .filter(|p| !p.as_os_str().is_empty())
        .take_while(|p| !written_dirs.contains(*p))
        .collect();
    pending.reverse();

    for dir in pending {
        zip_writer.add_directory(zip_name(dir), options)?;
        written_dirs.insert(dir.to_path_buf());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn archive_names(zip_path: &Path) -> Vec<String> {
        let archive = zip::ZipArchive::new(File::open(zip_path).unwrap()).unwrap();
        let mut names: Vec<String> = archive.file_names().map(String::from).collect();
        names.sort();
        names
    }

    #[test]
    fn test_excluded_contents_do_not_leave_empty_dirs() {
        let src = tempfile::tempdir().unwrap();
        let root = src.path();
        fs::create_dir_all(root.join("__pycache__")).unwrap();
        fs::write(root.join("__pycache__").join("a.cpython-311.pyc"), "x").unwrap();
        fs::create_dir_all(root.join("pkg").join("__pycache__")).unwrap();
        fs::write(root.join("pkg").join("__pycache__").join("b.cpython-311.pyc"), "x").unwrap();
        fs::write(root.join("pkg").join("mod.py"), "x").unwrap();
        fs::write(root.join("__init__.py"), "x").unwrap();
        fs::create_dir_all(root.join("empty")).unwrap();

        let out = tempfile::tempdir().unwrap();
        let zip_path = out.path().join("out.zip");
        let globs = crate::ops::glob_ops::compile_string_globs(&["**/__pycache__/**", "**/*.pyc"])
            .unwrap();
        build_archive(root, zip_path.as_path(), &globs).unwrap();

        assert_eq!(
            archive_names(&zip_path),
            vec!["__init__.py", "empty/", "pkg/", "pkg/mod.py"]
        );
    }
}

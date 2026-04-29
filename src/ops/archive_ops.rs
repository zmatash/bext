use std::{fs::File, io::BufWriter, path::Path};

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

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        let name = path.strip_prefix(src_dir)?;
        if name.as_os_str().is_empty() {
            continue;
        }

        let name_str = name.to_string_lossy().replace("\\", "/");

        if path.is_file() {
            zip_writer.start_file(name_str, options)?;
            let mut f = File::open(path)?;
            std::io::copy(&mut f, &mut zip_writer)?;
        } else if path.is_dir() {
            zip_writer.add_directory(name_str, options)?;
        }
    }

    zip_writer.finish()?;
    Ok(())
}

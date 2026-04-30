# bext

A command-line tool for building and managing Blender extensions.

## Installation

```bash
cargo install --path .
```

## Quick Start

1. Create `bext.toml` in your project root:

```toml
source_dir = "src"
output_dir = "build"
blender_versions = ["4.2", "4.1"]
exclude_globs = ["__pycache__", "*.pyc", ".git"]
```

2. Run commands:

```bash
bext build           # Package extension as ZIP
bext link            # Symlink for development
bext link --replace  # Symlink, replacing existing
bext unlink          # Remove symlinks
bext clean           # Delete excluded files
```

## Configuration

Create a `bext.toml` file in your project root. The tool searches up the directory tree to find it.

### Required

- **`source_dir`**: Directory containing your extension source. Must have `blender_manifest.toml`.

### Optional

- **`output_dir`**: Where to save built archives. Required for `build`.
- **`blender_versions`**: Blender versions to target (e.g., `["4.2", "4.1"]`). Required for `link`/`unlink`.
- **`exclude_globs`**: Patterns to exclude from archives (e.g., `["__pycache__", "*.pyc"]`). Required for `clean`.
- **`package_name`**: Custom ZIP filename. Supports `{name}`, `{version}`, `{id}`, `{maintainer}` placeholders.

### Example

```toml
source_dir = "src/my_extension"
output_dir = "dist"
blender_versions = ["4.2", "4.1"]
exclude_globs = ["__pycache__", "*.pyc"]
package_name = "{name}-{version}"
```

## Commands

### build

Packages your extension as a ZIP archive.

```bash
bext build
```

### link

Creates symlinks in Blender extension directories.

```bash
bext link [--replace]
```

Use `--replace` to overwrite existing symlinks.

### unlink

Removes symlinks from Blender extension directories.

```bash
bext unlink
```

### clean

Deletes files matching `exclude_globs` patterns.

```bash
bext clean
```

## File Structure

```
addon/
├── blender_manifest.toml  # Required
├── __init__.py
├── operators.py
└── properties.py
```

# bext

A command-line tool for building and managing Blender extensions.

## Overview

**bext** streamlines the development workflow for Blender extensions. It handles packaging extensions into ZIP archives, symlinking them into Blender's extension directories for development, and cleaning up build artifacts.

## Installation

Build from source:

```bash
cargo install --path .
```

## Quick Start

1. Create a `bext.toml` configuration file in your project root:

```toml
source_dir = "src"
output_dir = "build"
blender_versions = ["4.2", "4.1"]
exclude_globs = ["__pycache__", "*.pyc", ".git"]
```

2. Run commands to build and test your extension:

```bash
bext build    # Package extension as ZIP
bext link     # Symlink for development testing
bext unlink   # Remove symlinks
```

## Configuration

Create a `bext.toml` file in your project root. The tool searches up the directory tree from the current working directory to find this config file.

### Required Fields

- **`source_dir`** (string): Path to the directory containing your Blender extension source code. This directory must contain a `blender_manifest.toml` file.

### Optional Fields

- **`output_dir`** (string): Directory where built archives are saved. Created if it doesn't exist. Required for the `build` command.

- **`blender_versions`** (array of strings): List of Blender versions to target for symlinking. Versions can be specified as `"4.2"`, `"4.2.1"`, or `"4"`. Required for `link` and `unlink` commands.

- **`exclude_globs`** (array of strings): Glob patterns for files to exclude from the built archive (e.g., `__pycache__`, `*.pyc`, `.git`). Required for the `clean` command.

- **`package_name`** (string): Custom name for the generated ZIP archive. If not specified, uses the extension name from `blender_manifest.toml`.

Can use placeholders to include information from extension manifest, supported placeholders:
- `{name}`: Extension name from `blender_manifest.toml`
- `{version}`: Extension version from `blender_manifest.toml`
- `{id}`: Extension ID from `blender_manifest.toml`
- `{maintainer}`: Extension maintainer from `blender_manifest.toml`

### Example Configuration

```toml
source_dir = "src/my_extension"
output_dir = "dist"
blender_versions = ["4.2", "4.1", "4.0"]
exclude_globs = ["__pycache__", "*.pyc"]
package_name = "{name} - {version}"
```

## Commands

### build

Packages your extension into a ZIP archive for distribution.

```bash
bext build
```

**What it does:**
- Reads the `bext.toml` configuration
- Validates your extension structure
- Creates a compressed ZIP file in the output directory
- Excludes files matching the patterns in `exclude_globs`

**Requirements:**
- `output_dir` must be specified in `bext.toml`
- Extension must contain a valid `blender_manifest.toml`

**Output:**
- Creates `{package_name}.zip` in the output directory

### link

Creates symbolic links to your extension in Blender's extensions directory, allowing you to test changes without rebuilding.

```bash
bext link
```

**What it does:**
- Reads the `bext.toml` configuration
- For each Blender version in `blender_versions`, creates a symlink from your source directory to Blender's extensions folder
- Allows you to edit your extension and see changes in Blender immediately

**Requirements:**
- `blender_versions` must be specified in `bext.toml`
- Blender must be installed with those versions

**Platform Support:**
- Works on Windows (requires admin privileges or developer mode for symlinks)
- Works on macOS and Linux

### unlink

Removes the symbolic links created by the `link` command.

```bash
bext unlink
```

**What it does:**
- Removes symlinks from all Blender extension directories specified in `blender_versions`

**Use when:**
- You've finished development and want to clean up
- You're switching to a different extension directory structure

### clean

Deletes files matching the exclude patterns specified in your configuration.

```bash
bext clean
```

**What it does:**
- Scans your source directory for files matching the glob patterns in `exclude_globs`
- Deletes all matching files and directories
- Reports the number of items deleted

**Requirements:**
- `exclude_globs` must be specified in `bext.toml`

**Use when:**
- You want to remove build artifacts, cache files, or test data
- You're preparing to commit changes and want to clean up

## Typical Workflow

### Development Setup

```bash
# Create bext.toml configuration
# Initialize with your extension details

# Symlink for development
bext link

# Edit your extension files in your IDE
# Changes are reflected immediately in Blender
```

### Before Distribution

```bash
# Clean up artifacts
bext clean

# Build the distribution archive
bext build

# Verify the generated ZIP contains everything needed
```

### Cleanup

```bash
# When finished with development
bext unlink
```

## Configuration Examples

### Minimal Python Extension

```toml
source_dir = "addon"
output_dir = "dist"
blender_versions = ["4.2"]
exclude_globs = ["__pycache__", "*.pyc"]
```

### Complex Extension with Tests and Docs

```toml
source_dir = "src"
output_dir = "releases"
blender_versions = ["4.2", "4.1", "4.0"]
exclude_globs = [
    "__pycache__",
    "*.pyc",
    ".git",
    ".gitignore",
    "tests",
    "docs",
    "*.egg-info",
    ".vscode",
    ".pytest_cache"
]
package_name = "advanced_addon"
```

## Troubleshooting

### "Config file not found"

The tool couldn't find `bext.toml`. Make sure it exists in your project root or a parent directory.

### "Source path does not exist"

The `source_dir` specified in `bext.toml` doesn't exist. Check the path is correct and relative to where you run the command.

### "No Blender versions specified"

The `blender_versions` array in `bext.toml` is empty or missing. Add it for `link`/`unlink` commands.

### "No output directory specified"

The `output_dir` is missing from `bext.toml`. Add it for the `build` command.

### Symlinks not working on Windows

Windows requires either:
- Admin privileges
- Developer mode enabled (Windows 11+)

Run your terminal as Administrator or enable Developer Mode in Windows Settings.

## File Structure

Your extension directory should follow Blender's extension structure:
 
```
addon/
├── blender_manifest.toml    # Required - Blender extension manifest
├── __init__.py              # Python package initialization
├── operators.py             # Example: Your addon code
└── properties.py
```

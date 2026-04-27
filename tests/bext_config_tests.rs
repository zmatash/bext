use std::str::FromStr;

use bext::manifests::bext_config::BextConfig;
use semver::Version;

use crate::resources::TestResources;

mod resources;

#[test]
fn test_bext_config_template() {
    let resources = TestResources::new();
    let config = BextConfig::from_file(&resources.bext_config_template).unwrap();
    assert_eq!(config.source_dir, "src");
    assert_eq!(config.output_dir, "dist");
    assert_eq!(
        config.exclude_globs,
        Some(vec![
            "**/__pycache__/**".to_string(),
            "**/*.pyc".to_string()
        ])
    );
    assert_eq!(
        config.blender_versions,
        Some(vec![
            Version::from_str("4.2.0").unwrap(),
            Version::from_str("4.3.0").unwrap(),
        ])
    );
}

#[test]
fn test_bext_config_optional_fields_absent() {
    let config: BextConfig = "source_dir = \"src\"\noutput_dir = \"dist\"\n"
        .parse()
        .unwrap();
    assert_eq!(config.source_dir, "src");
    assert_eq!(config.output_dir, "dist");
    assert!(config.exclude_globs.is_none());
    assert!(config.blender_versions.is_none());
}

#[test]
fn test_bext_config_write_preservation() {
    let resources = TestResources::new();
    let mut config = BextConfig::from_file(&resources.bext_config_template).unwrap();
    let original_string = config.to_string().unwrap();

    config.output_dir = "build".to_string();
    let updated_string = config.to_string().unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let temp_path = tempdir.path().join("bext_config.toml");
    config.to_file(&temp_path).unwrap();

    let config_from_file = BextConfig::from_file(&temp_path).unwrap();
    let string_from_file = config_from_file.to_string().unwrap();
    assert_eq!(string_from_file, updated_string);
    assert_ne!(string_from_file, original_string);
    assert_eq!(config_from_file.output_dir, "build");
    assert_eq!(config_from_file.source_dir, "src");
}

#[test]
fn test_bext_config_optional_fields_roundtrip() {
    let resources = TestResources::new();
    let mut config = BextConfig::from_file(&resources.bext_config_template).unwrap();

    config.exclude_globs = None;
    config.blender_versions = None;

    let tempdir = tempfile::tempdir().unwrap();
    let temp_path = tempdir.path().join("bext_config.toml");
    config.to_file(&temp_path).unwrap();

    let config_from_file = BextConfig::from_file(&temp_path).unwrap();
    assert!(config_from_file.exclude_globs.is_none());
    assert!(config_from_file.blender_versions.is_none());
    assert_eq!(config_from_file.source_dir, "src");
    assert_eq!(config_from_file.output_dir, "dist");
}

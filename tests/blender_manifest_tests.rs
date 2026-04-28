use std::str::FromStr;

use bext::manifests::blender_manifest::BlenderManifest;
use semver::Version;

use crate::fixtures::TestResources;

mod fixtures;

#[test]
fn test_blender_manifest_template() {
    let resources = TestResources::new();
    let manifest = BlenderManifest::from_file(&resources.blender_manifest_template).unwrap();
    assert_eq!(manifest.id, "test_addon");
    assert_eq!(manifest.version, Version::from_str("0.1.0").unwrap());
    assert_eq!(manifest.name, "Test Addon");
    assert_eq!(manifest.maintainer, "Test Maintainer");
    assert_eq!(manifest.website, Some("https://example.com".to_string()));
    assert_eq!(
        manifest.blender_version_min,
        Some(Version::from_str("4.2.0").unwrap())
    );
    let perms = manifest.permissions.unwrap();
    assert_eq!(perms.files, Some("Read project files.".to_string()));
    assert_eq!(
        perms.network,
        Some("Send anonymized statistics.".to_string())
    );
}

#[test]
fn test_blender_manifest_write_preservation() {
    let resources = TestResources::new();
    let mut manifest = BlenderManifest::from_file(&resources.blender_manifest_template).unwrap();
    let original_string = manifest.to_string().unwrap();

    manifest.version = Version::from_str("1.0.0").unwrap();
    let updated_string = manifest.to_string().unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let temp_path = tempdir.path().join("blender_manifest.toml");
    manifest.to_file(&temp_path).unwrap();

    let manifest_from_file = BlenderManifest::from_file(&temp_path).unwrap();
    let string_from_file = manifest_from_file.to_string().unwrap();
    assert_eq!(string_from_file, updated_string);
    assert_ne!(string_from_file, original_string);
    assert_eq!(
        manifest_from_file.version,
        Version::from_str("1.0.0").unwrap()
    );
    assert_eq!(manifest_from_file.id, "test_addon");
    assert_eq!(manifest_from_file.name, "Test Addon");
}

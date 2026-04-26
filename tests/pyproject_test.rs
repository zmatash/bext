use std::str::FromStr;

use bext::manifests::pyproject::PyProject;
use pep440_rs::Version;

use crate::resources::TestResources;

mod resources;

#[test]
fn test_pyproject_template() {
    let resources = TestResources::new();
    let pyproject = PyProject::from_file(&resources.pyproject_template).unwrap();
    assert_eq!(pyproject.project.name, "test-package");
    assert_eq!(
        pyproject.project.version,
        Version::from_str("0.1.0").unwrap()
    );
}

#[test]
fn test_pyproject_write_preservation() {
    let resources = TestResources::new();
    let mut pyproject = PyProject::from_file(&resources.pyproject_template).unwrap();
    let pyproject_string = pyproject.to_string().unwrap();

    pyproject.project.version = Version::from_str("1.0.0").unwrap();
    let pyproject_string_updated = pyproject.to_string().unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let temp_path = tempdir.path().join("pyproject.toml");
    pyproject.to_file(&temp_path).unwrap();

    let pyproject_from_file = PyProject::from_file(&temp_path).unwrap();
    let pyproject_string_from_file = pyproject_from_file.to_string().unwrap();
    assert_eq!(pyproject_string_from_file, pyproject_string_updated);
    assert_ne!(pyproject_string_from_file, pyproject_string);
}

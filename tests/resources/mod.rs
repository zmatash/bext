use std::path::PathBuf;

#[allow(dead_code)]
pub struct TestResources {
    pub pyproject_template: PathBuf,
    pub blender_manifest_template: PathBuf,
}

impl TestResources {
    pub fn new() -> Self {
        let resources_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
        Self {
            pyproject_template: resources_root.join("pyproject_template.toml"),
            blender_manifest_template: resources_root.join("blender_manifest_template.toml"),
        }
    }
}

use std::path::PathBuf;

pub struct TestResources {
    pub pyproject_template: PathBuf,
}

impl TestResources {
    pub fn new() -> Self {
        Self {
            pyproject_template: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/resources/pyproject.toml"),
        }
    }
}

use ranote_core::{doc::Doc, error::Result};
use tempfile::TempDir;

#[test]
fn create_new_doc() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let doc = Doc::new(temp_dir.path())?;

    Ok(())
}

use ranote_core::{note::Note, error::Result};
use tempfile::TempDir;

#[test]
fn create_new_doc() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let doc = Note::new(temp_dir.path(), "tmp")?;

    drop(doc);

    let mut doc = Note::open(temp_dir.path().join("tmp"))?;
    assert_eq!(doc.content, "");

    doc.write("test content".to_owned())?;

    assert_eq!(doc.content.as_str(), "test content");

    Ok(())
}

use ranote_core::{error::Result, note::Note};
use tempfile::TempDir;

#[test]
fn create_new_note() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let note = Note::new(temp_dir.path(), "tmp")?;

    drop(note);

    let mut note = Note::open(temp_dir.path().join("tmp"))?;
    assert_eq!(note.get_content().unwrap(), "");

    note.write("test content".to_owned())?;

    assert_eq!(note.get_content().unwrap().as_str(), "test content");

    note.write("write again".to_owned())?;

    assert_eq!(note.get_content().unwrap().as_str(), "write again");
    Ok(())
}

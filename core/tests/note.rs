use core::{error::Result, note::Note};
use tempfile::TempDir;

#[test]
fn create_new_note() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let note = Note::new(temp_dir.path(), "tmp")?;

    drop(note);

    let tmp_file_path = temp_dir.path().join("tmp");
    let mut note = Note::open(&tmp_file_path)?;
    assert_eq!(note.content(), "");

    note.write("test content".to_owned())?;

    let fs_content = std::fs::read_to_string(&tmp_file_path).unwrap();

    assert_eq!(fs_content, "test content");
    assert_eq!(note.content(), "test content");

    note.write("abcd".to_owned())?;

    assert_eq!(note.content(), "abcd");
    Ok(())
}

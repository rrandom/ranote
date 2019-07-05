use crate::error::Result;
use crate::note::Note;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Wkspace {
    workspace_path: PathBuf,
    notes: BTreeMap<String, Note>,
    tags: BTreeSet<String>,
}

impl Wkspace {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Wkspace> {
        let wk_path = PathBuf::from(path.as_ref());

        let notes: BTreeMap<_, Note> = fs::read_dir(path)
            .expect("could not read dir")
            .map(|p| {
                let note_path = p.unwrap().path();

                (
                    note_path.to_str().unwrap().to_string(),
                    Note::open(note_path).unwrap(),
                )
            })
            .collect();

        Ok(Wkspace {
            workspace_path: wk_path,
            notes,
            tags: BTreeSet::new(),
        })
    }
}

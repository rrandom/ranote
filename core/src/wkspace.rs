use crate::error::Result;
use crate::note::{Note, NoteItem};
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
                let note = Note::open(note_path).expect("could not open note");

                (note.get_name().to_owned(), note)
            })
            .collect();

        Ok(Wkspace {
            workspace_path: wk_path,
            notes,
            tags: BTreeSet::new(),
        })
    }

    pub fn get_notes_names(&self) -> Result<Vec<NoteItem>> {
        let names: Vec<_> = self
            .notes
            .values()
            .map(|note| note.get_json_value().unwrap())
            .collect();
        Ok(names)
    }

    pub fn get_path(&self) -> Result<&Path> {
        Ok(self.workspace_path.as_ref())
    }

    pub fn get_note_by_name(&mut self, name: &str) -> Result<&mut Note> {
        self.notes
            .get_mut(name)
            .ok_or_else(|| failure::err_msg("no note"))
    }
}

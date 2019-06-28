use crate::note::Note;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

pub struct Wkspace<'note> {
    workspace_path: PathBuf,
    docs: BTreeMap<String, &'note Note>,
    tags: BTreeSet<String>,
    cates: BTreeMap<String, &'note Note>,
}

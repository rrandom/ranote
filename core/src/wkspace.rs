use crate::note::Note;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

pub struct Wkspace {
    workspace_path: PathBuf,
    docs: BTreeMap<String, Note>,
    tags: BTreeSet<String>,
    cates: BTreeMap<String, Note>,
}

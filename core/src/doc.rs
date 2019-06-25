use crate::error::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub struct Doc {
    path: PathBuf,
    name: String,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    tags: BTreeSet<String>,
}

impl Doc {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        unimplemented!();
    }

    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        unimplemented!();
    }

    pub fn read() -> Result<()> {
        unimplemented!();
    }

    pub fn write() -> Result<()> {
        unimplemented!();
    }

    pub fn add_tag() -> Result<()> {
        unimplemented!();
    }

    pub fn rm_tag() -> Result<()> {
        unimplemented!();
    }
}

pub struct DocList {
    docs: BTreeMap<String, Doc>,
    tags: BTreeSet<String>,
}

use crate::error::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};

pub struct Doc {
    path: PathBuf,
    name: String,
    content: String,
    writer: BufWriter<File>,
    tags: BTreeSet<String>,
}

impl Doc {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        unimplemented!();
    }

    pub fn new<T: AsRef<Path>>(path: T, name: &str) -> Result<Self> {
        let path = path.as_ref().join(name);

        // TO-DO: check if file exists
        let f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)?;

        let writer = BufWriter::new(f);

        Ok(Doc {
            path: PathBuf::from(path),
            name: String::from(name),
            content: String::from(""),
            writer,
            tags: BTreeSet::new(),
        })
    }

    pub fn refresh(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn read(&mut self) -> Result<()> {
        let mut f = File::open(self.path.as_path())?;
        let mut content = String::from("");

        f.read_to_string(&mut content)?;
        self.content = content;

        Ok(())
    }

    pub fn write(&mut self, content: String) -> Result<()> {
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

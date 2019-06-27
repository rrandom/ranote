use crate::error::Result;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use failure::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UiDoc {
    path: String,
    name: String,
}

pub struct Doc {
    path: PathBuf,
    name: String,
    pub content: String,
    writer: BufWriter<File>,
    tags: BTreeSet<String>,
}

impl Doc {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            unreachable!();
        }

        let f = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&path)?;

        let writer = BufWriter::new(f);
        let name = path.file_name().unwrap().to_os_string();

        let mut doc = Doc {
            path: PathBuf::from(path),
            name: name.into_string().unwrap(),
            content: String::from(""),
            writer,
            tags: BTreeSet::new(),
        };

        doc.read()?;

        Ok(doc)
    }

    pub fn new<T: AsRef<Path>>(path: T, name: &str) -> Result<Self> {
        let path = path.as_ref().join(name);

        if path.exists() {
            unreachable!();
        }

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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_json_value(&self) -> Result<UiDoc> {
        Ok(UiDoc {
            path: String::from(self.path.to_str().unwrap()),
            name: self.name.clone()
        })
    }

    pub fn get_content(&self) -> Result<String> {
        Ok(self.content.clone())
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
        write!(self.writer, "{}", &content)?;
        self.content = content;
        Ok(())
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
    workspace_path: PathBuf,
}

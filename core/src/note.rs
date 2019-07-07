use crate::error::Result;
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Read, Seek, SeekFrom, Write, BufReader};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct NoteItem {
    path: String,
    name: String,
}

pub struct Note {
    path: PathBuf,
    name: String,
    content: String,
    writer: BufWriter<File>,
    reader: BufReader<File>,
    tags: BTreeSet<String>,
}

impl Note {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            unreachable!();
        }

        let wf = OpenOptions::new().write(true).read(true).open(&path)?;
        let rf = wf.try_clone()?;

        let writer = BufWriter::new(wf);
        let reader = BufReader::new(rf);
        let name = path.file_name().unwrap().to_os_string();

        let mut note = Note {
            path: PathBuf::from(path),
            name: name.into_string().unwrap(),
            content: String::from(""),
            writer,
            reader,
            tags: BTreeSet::new(),
        };

        note.read()?;

        Ok(note)
    }

    pub fn new<T: AsRef<Path>>(path: T, name: &str) -> Result<Self> {
        let path = path.as_ref().join(name);

        if path.exists() {
            unreachable!();
        }

        let wf = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)?;
        let rf = wf.try_clone()?;

        let writer = BufWriter::new(wf);
        let reader = BufReader::new(rf);

        Ok(Note {
            path,
            name: String::from(name),
            content: String::from(""),
            writer,
            reader,
            tags: BTreeSet::new(),
        })
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_path(&self) -> String {
        String::from(self.path.to_str().unwrap())
    }

    pub fn get_json_value(&self) -> Result<NoteItem> {
        Ok(NoteItem {
            path: self.get_path(),
            name: self.name.clone(),
        })
    }

    pub fn get_content(&self) -> Result<String> {
        Ok(self.content.clone())
    }

    pub fn refresh(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn read(&mut self) -> Result<()> {
        let mut content = String::from("");
        self.reader.seek(SeekFrom::Start(0))?;
        self.reader.read_to_string(&mut content)?;
        self.content = content;

        Ok(())
    }

    pub fn write(&mut self, content: String) -> Result<()> {
        self.writer.seek(SeekFrom::Start(0))?;
        self.writer.write_all(&content.as_bytes())?;
        self.writer.flush()?;
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

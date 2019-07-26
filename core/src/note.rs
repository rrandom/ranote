use crate::error::Result;
use front::{get_note_content, NoteMetaData};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteItem {
    path: String,
    name: String,
    id: String,
}

#[derive(Debug)]
pub struct Note {
    path: PathBuf,
    id: String,
    content: String,
    writer: BufWriter<File>,
    reader: BufReader<File>,
    pub meta: NoteMetaData,
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
        let id = path.file_name().unwrap().to_os_string();

        let mut note = Note {
            path: PathBuf::from(path),
            id: id.into_string().unwrap(),
            content: String::from(""),
            writer,
            reader,
            meta: NoteMetaData::default(),
            tags: BTreeSet::new(),
        };

        note.read()?;

        Ok(note)
    }

    pub fn new<T: AsRef<Path>>(path: T, name: &str) -> Result<Self> {
        let path = path.as_ref().join(name);

        if path.exists() {
            failure::err_msg("file already exists");
        }

        std::fs::File::create(&path)?;
        let note = Note::open(path)?;

        Ok(note)
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.meta.title()
    }

    pub fn get_path(&self) -> String {
        String::from(self.path.to_str().unwrap())
    }

    pub fn get_json_value(&self) -> Result<NoteItem> {
        Ok(NoteItem {
            path: self.get_path(),
            name: String::from(self.id()),
            id: self.id.clone(),
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
        let (meta, content) = get_note_content(&self.path, &content)
            .unwrap_or_else(|_| (NoteMetaData::default(), content));
        self.meta = meta;
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

    pub fn save(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn add_tag() -> Result<()> {
        unimplemented!();
    }

    pub fn rm_tag() -> Result<()> {
        unimplemented!();
    }
}

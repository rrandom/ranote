use front::{get_note_content, NoteMetaData};
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::error::*;
use snafu::*;

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
}

impl Note {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return PathNotExist {
                path: PathBuf::from(path),
            }
            .fail();
        }

        let wf = OpenOptions::new()
            .write(true)
            .read(true)
            .open(&path)
            .context(IoError {
                path: PathBuf::from(path),
            })?;
        let rf = wf.try_clone().context(IoError {
            path: PathBuf::from(path),
        })?;

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
        };

        note.read()?;

        Ok(note)
    }

    pub fn new<T: AsRef<Path>>(path: T, name: &str) -> Result<Self> {
        let path = path.as_ref().join(name);

        if path.exists() {
            NoteAlreadlyExist { path: path.clone() }.fail()?;
        }

        fs::File::create(&path).context(IoError { path: path.clone() })?;
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
            name: String::from(self.name()),
            id: self.id.clone(),
        })
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn refresh(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn read(&mut self) -> Result<()> {
        let mut content = String::from("");
        self.reader.seek(SeekFrom::Start(0)).context(IoError {
            path: self.path.clone(),
        })?;
        self.reader.read_to_string(&mut content).context(IoError {
            path: self.path.clone(),
        })?;
        let (meta, content) = get_note_content(&self.path, &content).unwrap_or_else(|e| {
            dbg!(e);
            (NoteMetaData::default(), content)
        });
        self.meta = meta;
        self.content = content;

        Ok(())
    }

    pub fn write(&mut self, content: String) -> Result<bool> {
        // self.writer.seek(SeekFrom::Start(0))?;
        // self.writer.write_all(&content.as_bytes())?;
        // self.writer.flush()?;
        let mut update_ui = false;
        self.content = content.clone();

        let tcontent = content.trim();
        if !tcontent.is_empty() {
            let title = content.lines().nth(0).unwrap();
            self.meta.set_title(title);
            update_ui = true;
        }
        let meta_str = self.meta.to_string()?;
        let content = format!("+++\n{}+++\n{}", meta_str, content);

        std::fs::write(&self.path, &content).context(IoError {
            path: self.path.clone(),
        })?;
        Ok(update_ui)
    }

    pub fn save(&mut self) -> Result<()> {
        unimplemented!();
    }

    pub fn add_tag(&mut self, tag: &str) -> Result<()> {
        self.meta.add_tag(tag);
        Ok(())
    }

    pub fn rm_tag(&mut self, tag: &str) -> Result<()> {
        self.meta.remove_tag(tag);
        Ok(())
    }
}

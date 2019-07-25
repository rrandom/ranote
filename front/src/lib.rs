#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::error::Error;
use std::path::Path;
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref PAGE_RE: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

fn split_content(file_path: &Path, content: &str) -> Result<(String, String)> {
    if !PAGE_RE.is_match(content) {
        println!(
            "Couldn't find front matter in `{}`. Did you forget to add `+++`?",
            file_path.to_string_lossy()
        );
    }

    let caps = PAGE_RE.captures(content).unwrap();
    Ok((caps[1].to_string(), caps[2].to_string()))
}

pub fn get_page_content(file_path: &Path, content: &str) -> Result<(NoteMetaData, String)> {
    let (front_matter, content) = split_content(file_path, content)?;
    let meta = NoteMetaData::parse(&front_matter)?;
    Ok((meta, content))
}

pub struct NoteMetaData {
    attachments: Vec<String>,
    created: String,
    modified: String,
    favorited: bool,
    pinned: bool,
    tags: Vec<String>,
    title: String,
}

impl NoteMetaData {
    fn parse(toml: &str) -> Result<NoteMetaData> {
        unimplemented!();
        
    }
}

impl Default for NoteMetaData {
    fn default() -> NoteMetaData {
        NoteMetaData {
            attachments: vec![],
            created: String::from(""),
            modified: String::from(""),
            favorited: false,
            pinned: false,
            tags: vec![],
            title: String::from(""),
        }
    }
}

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod error;

use regex::Regex;
use std::path::Path;

use error::*;
use snafu::*;

lazy_static! {
    static ref PAGE_RE: Regex =
        Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n?((?s).*(?-s))$").unwrap();
}

fn split_content(file_path: &Path, content: &str) -> Result<(String, String)> {
    if !PAGE_RE.is_match(content) {
        return Err(Error::NoFrontMatter {
            path: format!("{}", file_path.to_string_lossy()),
        });
    }

    let caps = PAGE_RE.captures(content).unwrap();
    Ok((caps[1].to_string(), caps[2].to_string()))
}

pub fn get_note_content(file_path: &Path, content: &str) -> Result<(NoteMetaData, String)> {
    let (front_matter, content) = split_content(file_path, content)?;
    let meta = NoteMetaData::parse(&front_matter)?;
    dbg!(&meta);
    Ok((meta, content))
}

#[derive(Debug, Deserialize)]
pub struct NoteMetaData {
    attachments: Option<Vec<String>>,
    created: String,
    modified: String,
    favorited: bool,
    pinned: bool,
    tags: Option<Vec<String>>,
    title: String,
}

impl NoteMetaData {
    fn parse(toml_str: &str) -> Result<NoteMetaData> {
        let note_meta: NoteMetaData = toml::from_str(toml_str).context(ParseError {})?;
        Ok(note_meta)
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl Default for NoteMetaData {
    fn default() -> NoteMetaData {
        NoteMetaData {
            attachments: None,
            created: String::from(""),
            modified: String::from(""),
            favorited: false,
            pinned: false,
            tags: None,
            title: String::from(""),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_invalid() {
        let content = r#"
    title = "Hello"
    do_not_exist = "hey there""#;
        let res = NoteMetaData::parse(content);
        assert!(res.is_err());
    }

    #[test]
    fn test_process_note() {
        let content = r#"
+++
    title = "title"
    created = "String"
    modified = "String"
    favorited = false
    pinned = false
    +++
content string"#;
        let res = get_note_content(Path::new(""), content).expect("should get_content");
        let metadata = res.0;
        let content = res.1;

        assert_eq!(content, "content string");
        assert_eq!(metadata.modified, String::from("String"));
        assert_eq!(metadata.title, String::from("title"));
        assert_eq!(metadata.tags, None);
        assert_eq!(metadata.attachments, None);

        let content = r#"
+++
    title = "title"
    created = "String"
    modified = "String"
    favorited = false
    pinned = false
    tags = ["tag1"]
    attachments= ["attach1"]
    +++
content string"#;

        let res = get_note_content(Path::new(""), content).expect("should get_content");
        let metadata = res.0;
        let content = res.1;

        assert_eq!(content, "content string");
        assert_eq!(metadata.modified, String::from("String"));
        assert_eq!(metadata.title, String::from("title"));
        assert_eq!(metadata.tags, Some(vec![String::from("tag1")]));
        assert_eq!(metadata.attachments, Some(vec![String::from("attach1")]));
    }
}

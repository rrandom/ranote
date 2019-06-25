use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::error::Result;

pub struct Doc {
    path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    tags: BTreeSet<String>,
}

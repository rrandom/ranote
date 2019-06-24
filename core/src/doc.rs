use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub struct Doc {
    path: PathBuf,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    tags: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
}

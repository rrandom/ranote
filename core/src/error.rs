use snafu::*;
use std::path::PathBuf;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("IO error {}: {}", path.display(), source))]
    IoError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("path not exist {}", path.display()))]
    PathNotExist { path: PathBuf },

    #[snafu(display("note not exist {}", fname))]
    NoNote { fname: String },

    #[snafu(display("note already exist {}", path.display()))]
    NoteAlreadlyExist { path: PathBuf },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

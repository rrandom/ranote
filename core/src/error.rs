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

    MetaError{ source: front::error::Error }
}


impl From<front::error::Error> for Error {
    fn from(e: front::error::Error) -> Self {
        Error::MetaError {
            source: e
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

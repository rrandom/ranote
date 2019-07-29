use snafu::*;
use std::path::PathBuf;
use toml;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Could not open con  fig from {}: {}", filename.display(), source))]
    OpenConfig {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not save config to {}: {}", filename.display(), source))]
    SaveConfig {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("The user id {} is invalid", user_id))]
    UserIdInvalid { user_id: i32, backtrace: Backtrace },
    Any {
            detail: String,
        },

    #[snafu(display("{}", message))]
    MiscError { message: String },

    #[snafu(display("IO error {}: {}", path.display(), source))]
    IoError {path: PathBuf, source: toml::de::Error},
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

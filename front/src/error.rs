use snafu::*;
use toml;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("could not parse {}", source))]
    ParseError { source: toml::de::Error },

    #[snafu(display(
        "Couldn't find front matter in `{}`. Did you forget to add `+++`?",
        path
    ))]
    NoFrontMatter { path: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

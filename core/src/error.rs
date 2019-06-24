use std::result::{self};
use failure::{Error};

pub type Result<T> = result::Result<T, Error>;


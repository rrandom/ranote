use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Snafu)]
enum Error {
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
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn log_in_user<P>(config_root: P, user_id: i32) -> Result<bool>
where
    P: AsRef<Path>,
{
    let config_root = config_root.as_ref();
    let filename = &config_root.join("config.toml");

    let config = fs::read(filename).context(OpenConfig { filename })?;
    // Perform updates to config
    fs::write(filename, config).context(SaveConfig { filename })?;

    ensure!(user_id == 42, UserIdInvalid { user_id });

    Ok(true)
}

fn log_in() {
    match log_in_user("abcd", 11) {
        Ok(true) => println!("Logged in!"),
        Ok(false) => println!("Not logged in!"),
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            if let Some(backtrace) = ErrorCompat::backtrace(&e) {
                println!("{}", backtrace);
            }
        }
    }
}

fn main() {
    log_in()
}

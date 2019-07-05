use dirs::home_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// TO-DO: params is a serde-json string,
pub fn format_callback(callback: &str, params: &str) -> String {
    format!("{}({})", callback, params)
}

pub fn create_config() -> PathBuf {
    let mut config_file = home_dir().expect("expect home");
    config_file.push("ranote-config.toml");
    if config_file.exists() {
        config_file
    } else {
        File::create(&config_file)
            .expect("create config file")
            .write_all(b"")
            .unwrap();
        config_file
    }
}

pub fn get_wk_dir() -> PathBuf {
    let osstr = dirs::home_dir()
        .and_then(|mut h| {
            h.push(".ranote");
            Some(h.into_os_string())
        })
        .expect("cant get wkdir");

    PathBuf::from(osstr)
}

use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn read_file(file: String) -> String {
    let path = file.clone();
    let mut file = File::open(file).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

pub fn get_files() -> Vec<OsString> {
    let path = dirs::home_dir()
        .and_then(|mut h| {
            h.push(".ranote");
            Some(h.into_os_string())
        })
        .unwrap();

    dbg!(&path);

    if fs::metadata(&path).is_err() {
        fs::create_dir(&path).unwrap();
    }

    let files: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|k| k.unwrap().path().into_os_string())
        .collect();

    dbg!(&files);
    files
}

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs;
use std::ffi::{OsString};

fn get_files() -> Vec<OsString> {
    let path = dirs::home_dir()
        .and_then(|mut h| {
            h.push(".ranote");
            Some(h.into_os_string())
        })
        .unwrap();

    dbg!(&path);

    if let Err(_) = fs::metadata(&path) {
        fs::create_dir(&path).unwrap();
    }

    let files: Vec<_> = fs::read_dir(path)
        .unwrap()
        .map(|k| k.unwrap().path().into_os_string())
        .collect();

    dbg!(&files);
    files
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Read { text: String },
    TestClick { cb: String },
}

fn main() {
    let files = get_files();

    web_view::builder()
        .title("Ranote")
        // .content(Content::Html(include_str!("web/index.html")))
        .content(web_view::Content::Url("http://localhost:8081"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            match serde_json::from_str(arg).unwrap() {
                Init => {
                    println!("ui inited");
                    webview.eval(&format!("list_dir({})", 123)).unwrap();
                },
                Read { text } => println!("{}", text),
                TestClick { cb } => {
                    println!("TestClick");
                    webview.eval(&format!("{}()", cb)).unwrap();
                }
            }
            Ok(())
        })
        .run()
        .unwrap();
}

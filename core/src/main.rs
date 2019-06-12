#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde_json::json;

mod cmd;
mod file;
mod utils;
use cmd::Cmd;
use utils::format_callback;
use std::fs::File;
use std::io::Write;

fn main() {
    let files = file::get_files();

    web_view::builder()
        .title("Ranote")
        // .content(Content::Html(include_str!("web/index.html")))
        .content(web_view::Content::Url("http://localhost:8081"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|wv, arg| {
            use Cmd::*;

            match serde_json::from_str(arg).unwrap() {
                Init => {
                    println!("ui inited");
                    let files = json!(files);
                    wv.eval(&format_callback("listDir", &files.to_string()))
                        .unwrap();
                }
                Read { text } => println!("{}", text),
                SaveFile { file, contents } => {
                    let mut f = File::create(file).unwrap();
                    f.write_all(contents.as_bytes());
                }
                TestClick { cb } => {
                    println!("TestClick");
                    wv.eval(&format!("{}()", cb)).unwrap();
                }
                LoadFile { fileName, cb } => {
                    println!("{}", fileName);
                    let contents = file::read_file(fileName.clone());
                    let params = json!({ "name": fileName, "contents": contents });
                    println!("{}", params);

                    wv.eval(&format_callback(&cb, &params.to_string())).unwrap();
                }
                _ => {
                    unimplemented!();
                }
            }
            Ok(())
        })
        .run()
        .unwrap();
}

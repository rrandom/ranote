#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use serde_json::json;

mod cmd;
mod error;
mod file;
mod utils;

use cmd::Cmd;
use error::Result;
use std::fs::File;
use std::io::Write;
use utils::format_callback;

fn main() -> Result<()> {
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

            match serde_json::from_str(arg).expect("Could not get command") {
                Init => {
                    println!("ui inited");
                    let files = json!(files);
                    wv.eval(&format_callback("listDir", &files.to_string()))?;
                }
                Read { text } => println!("{}", text),
                SaveFile { file, contents } => {
                    let mut f = File::create(file).expect("Could not create file");
                    f.write_all(contents.as_bytes());
                }
                TestClick { cb } => {
                    println!("TestClick");
                    wv.eval(&format!("{}()", cb))?;
                }
                LoadFile { fileName, cb } => {
                    println!("{}", fileName);
                    let contents = file::read_file(fileName.clone());
                    let params = json!({ "name": fileName, "contents": contents });
                    println!("{}", params);

                    wv.eval(&format_callback(&cb, &params.to_string()))?;
                }
                _ => {
                    unimplemented!();
                }
            }
            Ok(())
        })
        .run()?;

    Ok(())
}

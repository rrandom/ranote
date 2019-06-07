#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cmd;
mod file;
mod utils;
use cmd::Cmd;

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
        .invoke_handler(|webview, arg| {
            use Cmd::*;

            match serde_json::from_str(arg).unwrap() {
                Init => {
                    println!("ui inited");
                    webview.eval(&format!("list_dir({})", 123)).unwrap();
                }
                Read { text } => println!("{}", text),
                TestClick { cb } => {
                    println!("TestClick");
                    webview.eval(&format!("{}()", cb)).unwrap();
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

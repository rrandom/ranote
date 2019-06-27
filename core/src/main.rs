use ranote_core::{cmd::Cmd, error::Result, file, utils::format_callback};
use serde_json::json;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let notes = file::get_files();

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

                    let file_names: Vec<_> =
                        notes.iter().map(|f| f.get_json_value().unwrap()).collect();
                    let files = serde_json::to_string(&file_names).unwrap();
                    wv.eval(&format_callback("listDir", &files.to_string()))?;
                }
                SaveNote { file, content } => {
                    let mut f = File::create(file).expect("Could not create file");
                    f.write_all(content.as_bytes()).unwrap();
                }
                LoadNote { path, cb } => {
                    dbg!(&path);
                    let contents = file::read_file(path.clone());
                    let params = json!({ "path": path, "content": contents });
                    // println!("{}", params);

                    wv.eval(&format_callback(&cb, &params.to_string()))?;
                }
                TestClick { cb } => {
                    println!("TestClick");
                    wv.eval(&format!("{}()", cb))?;
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

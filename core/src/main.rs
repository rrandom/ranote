use ranote_core::{cmd::Cmd, error::Result, utils, wkspace::Wkspace};
use serde_json::json;
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let wkspace_path = utils::get_wk_dir();
    let wkspace = Wkspace::open(wkspace_path)?;

    web_view::builder()
        .title("Ranote")
        // .content(Content::Html(include_str!("web/index.html")))
        .content(web_view::Content::Url("http://localhost:8085"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|wv, arg| {
            use Cmd::*;
            use utils::format_callback;

            match serde_json::from_str(arg).expect("Could not get command") {
                Init => {
                    dbg!("ui inited");

                    let notes = wkspace.get_notes_names().expect("could not get names");
                    let notes = serde_json::to_string(&notes).unwrap();
                    wv.eval(&format_callback("listDir", &notes.to_string()))?;
                }
                SaveNote { file, content } => {
                    let mut f = File::create(file).expect("Could not create file");
                    f.write_all(content.as_bytes()).unwrap();
                }
                LoadNote { name, cb } => {
                    let note = wkspace.get_note_by_name(&name).expect("could not get note");
                    let content = note.get_content().expect("no content");
                    let params = json!({ name: note.get_name(), "path": note.get_path(), "content": content });
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

use ranote_core::{cmd::Cmd, error::Result, utils, wkspace::Wkspace};
use serde_json::json;

#[macro_use]
extern crate slog;
extern crate slog_term;
use slog::Drain;
use std::sync::Mutex;

fn main() -> Result<()> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = Mutex::new(slog_term::FullFormat::new(decorator).build()).fuse();
    let root_log = slog::Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));

    let wkspace_path = utils::get_wk_dir();
    let mut wkspace = Wkspace::open(wkspace_path)?;

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
                Init { cb } => {
                    info!(root_log, "ui inited");
                    let notes = wkspace.get_notes_names().expect("could not get names");
                    let notes = serde_json::to_string(&notes).unwrap();
                    wv.eval(&format_callback(&cb, &notes.to_string()))?;
                }
                SaveNote { name, content } => {
                    let note = wkspace.get_note_by_name(&name).expect("could not get note");
                    note.write(content).expect("can not write");
                    info!(root_log, "Note Saved"; "name" => &name);
                }
                LoadNote { name, cb } => {
                    let note = wkspace.get_note_by_name(&name).expect("could not get note");
                    note.read().expect("refresh");
                    let content = note.get_content().expect("no content");
                    let params = json!({ "name": note.name(), "path": note.get_path(), "content": content });
                    wv.eval(&format_callback(&cb, &params.to_string()))?;
                    info!(root_log, "Note Loaded"; "name" => &name);
                }
                NewNote{ cb } => {
                    let note_name = wkspace.new_note().expect("create new note");
                    let params = json!({ "name": note_name});
                    wv.eval(&format_callback(&cb, &params.to_string()))?;
                    info!(root_log, "Note newed"; "name" => &note_name);
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

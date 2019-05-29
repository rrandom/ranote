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

fn main() {
    let files = get_files();

    web_view::builder()
        .title("Ranote")
        // .content(Content::Html(include_str!("web/index.html")))
        .content(web_view::Content::Url("http://localhost:8080"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "init" => {
                    println!("ui inited");
                    webview.eval(&format!("list_dir({})", 123)).unwrap();
                }
                "read" => {
                    println!("reading file!");
                    // let file_content = include_str!("web/index.html");
                    let file_content = "abcd";
                    println!("{}", file_content);
                    webview
                        .eval(&format!("file_operation({})", file_content))
                        .unwrap();
                }
                "test-click" => {
                    println!("test-click");
                }
                _ => unimplemented!(),
            };
            Ok(())
        })
        .run()
        .unwrap();
}

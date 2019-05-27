
use std::{fs};

fn main() {

    let path = dirs::home_dir().and_then(|mut h| {
        h.push(".ranote");
        Some(h.into_os_string())
    }).unwrap();

    dbg!(&path);

    if let Err(_) = fs::metadata(&path) {
        fs::create_dir(&path).unwrap();
    }

    let files: Vec<_> = fs::read_dir(path).unwrap().collect();

    dbg!(&files);

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
                "read" => {
                    println!("reading file!");
                    // let file_content = include_str!("web/index.html");
                    let file_content = "abcd";
                    println!("{}", file_content);
                    webview
                        .eval(&format!("file_operation({})", file_content))
                        .unwrap();
                }
                _ => unimplemented!(),
            };
            Ok(())
        })
        .run()
        .unwrap();
}

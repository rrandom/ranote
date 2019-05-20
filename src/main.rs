extern crate web_view;

use web_view::*;

fn main() {
    web_view::builder()
        .title("Juggernaut")
        // .content(Content::Html(include_str!("web/index.html")))
        .content(Content::Url("http://localhost:8081"))
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

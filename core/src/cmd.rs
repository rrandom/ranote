#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Read {
        text: String,
    },
    Write {
        file: String,
        contents: String,
    },
    List {
        path: String,
        cb: String,
    },
    ListDirs {
        cb: String,
        home: bool,
        path: String,
    },
    SetHome {
        cb: String,
        path: String,
    },
    TestClick {
        cb: String,
    },
}
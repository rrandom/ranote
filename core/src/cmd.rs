#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init { cb: String },
    SaveNote { id: String, content: String, cb: String },
    LoadNote { id: String, cb: String },
    NewNote { cb: String },
    Debug { name: String, msg: String },
    SetHome { cb: String, path: String },
    TestClick { cb: String },
}

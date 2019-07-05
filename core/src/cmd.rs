#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    SaveNote {
        name: String,
        content: String,
    },
    LoadNote {
        name: String,
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

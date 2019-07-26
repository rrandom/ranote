#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init {
        cb: String,
    },
    SaveNote {
        name: String,
        content: String,
    },
    LoadNote {
        name: String,
        cb: String,
    },
    NewNote {
        cb: String,
    },
    ListDirs {
        cb: String,
        home: bool,
        path: String,
    },
    Debug {
        msg: String,
    },
    SetHome {
        cb: String,
        path: String,
    },
    TestClick {
        cb: String,
    },
}

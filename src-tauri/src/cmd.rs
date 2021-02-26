use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Connect {
        address: String,
        callback: String,
        error: String,
    },
    GetPortInfo {
        callback: String,
        error: String,
    },
}

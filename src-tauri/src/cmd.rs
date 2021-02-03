use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  ExecuteCommand { argument: String },
  PerformRequest {
    data: String,
    callback: String,
    error: String,
  },
}

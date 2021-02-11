use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  StartCommunication { 
    baud: i32,
    port: String,
    callback: String,
    error: String,  
  },
  GetPortInfo {
    callback: String,
    error: String,
  },
}

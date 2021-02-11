#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
mod crawl;

use cmd::Cmd::*;


fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            StartCommunication { address, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                println!("{}", address);
                Ok("Success")
              },
              callback,
              error,
            ),
            GetPortInfo { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                Ok("{ key: 'response', value: [{ id: 3 }] }".to_string())
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}

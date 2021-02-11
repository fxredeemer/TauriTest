#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate serde;

mod serial;
mod cmd;
use cmd::Cmd::*;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            StartCommunication { baud, port, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                println!("{} {}", baud, port);
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

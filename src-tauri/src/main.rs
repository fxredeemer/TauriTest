#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use cmd::Cmd::*;
use serde::Serialize;
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct Reply {
    data: String,
}

fn main() {
    tauri::AppBuilder::new()
        .setup(|webview, _| {
            setup_event_to_js(webview);
        })
        .invoke_handler(|webview, arg| handle_invocation(webview, arg))
        .build()
        .run();
}

fn handle_invocation(webview: &mut tauri::Webview, arg: &str) -> Result<(), String> {
    match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
            execute_command(webview, command);
            Ok(())
        }
    }
}

fn setup_event_to_js(webview: &mut tauri::Webview) {
    let mut webview = webview.as_mut();
    thread::spawn(move || loop {
        let reply = Reply {
            data: "something else".to_string(),
        };

        thread::sleep(Duration::from_secs(1));
        tauri::event::emit(&mut webview, String::from("rust-event"), Some(reply))
            .expect("failed to emit");
    });
}

fn execute_command(_webview: &mut tauri::Webview, command: cmd::Cmd) -> () {
    match command {
        Connect {
            address,
            callback,
            error,
        } => tauri::execute_promise(
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
            move || Ok("{ key: 'response', value: [{ id: 3 }] }".to_string()),
            callback,
            error,
        ),
    }
}

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;

use cmd::Cmd::*;
use futures::executor::block_on;
use futures::prelude::*;
use irc::client::prelude::*;

fn main() {
    block_on(start_chat_async());

    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| match serde_json::from_str(arg) {
            Err(e) => Err(e.to_string()),
            Ok(command) => {
                execute_command(_webview, command);
                Ok(())
            }
        })
        .build()
        .run();
}

async fn start_chat_async() {
    // configuration is loaded from config.toml into a Config
    let mut client = Client::new("../config.toml").await.unwrap();

    // identify comes from ClientExt
    client.identify().unwrap();
    let mut stream = client.stream().unwrap();
    while let Some(message) = stream.next().await.transpose().unwrap() {
        if let Command::PRIVMSG(channel, message) = message.command {
            if message.contains(&*client.current_nickname()) {
                // send_privmsg comes from ClientExt
                client.send_privmsg(&channel, "beep boop").unwrap();
            }
        }
    }
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

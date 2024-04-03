// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

mod cmd;
mod menu;
mod meta;
mod utils;

use command_group::CommandGroup;
use std::process::Command;
use std::sync::mpsc::{sync_channel, Receiver};
use std::thread;
use tauri::api::process::Command as TCommand;

use tauri::{RunEvent, WindowEvent};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};

#[cfg(debug_assertions)]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];

#[cfg(debug_assertions)]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Error;

fn main() {
    // Start the python server
    let (tx, rx) = sync_channel(1);
    run_backend(rx);

    let mut builder = tauri::Builder::default();
    let mut tauri_ctx = tauri::generate_context!();

    let _app_config = utils::config::AppConfig::load();

    // register tauri plugins
    builder = builder
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .with_colors(ColoredLevelConfig::default())
                .level_for("tauri", log::LevelFilter::Info)
                .level_for("hyper", log::LevelFilter::Off)
                .level_for("sqlx::query", log::LevelFilter::Off)
                .level(LOG_LEVEL)
                .build(),
        )
        .plugin(tauri_plugin_positioner::init())
        .plugin(plugin_window_theme::ThemePlugin::init(
            tauri_ctx.config_mut(),
        ));

    // setup and create window
    builder = builder.setup(|app| {
        // Set activation policy to `Accessory` to prevent
        // the app icon from showing on the dock.
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Regular);

        // Create main window for the application.
        utils::webview::create_window(&app.handle(), meta::MAIN_WINDOW, "index.html");

        log::info!("Platform: {}-{}", meta::PKG_OS, meta::PKG_ARCH);

        Ok(())
    });

    // setup window menu
    builder = builder
        .enable_macos_default_menu(false)
        .menu(menu::build_app_menu())
        .on_menu_event(menu::app_menu_event);

    // Terminate the server before closing the main application
    // Tell the child process to shutdown when app exits
    builder = builder.on_window_event(move |event| match event.event() {
        WindowEvent::Destroyed => {
            tx.send(-1).expect("[Error] sending msg.");
            println!("[Event] App closed, shutting down API...");
        }
        _ => {}
    });

    // run the application
    builder
        .invoke_handler(tauri::generate_handler![
            cmd::general::open_devtools,
            cmd::general::get_machine_id,
            cmd::general::create_child_window,
            cmd::general::open_settings_window,
            cmd::general::set_darkmode,
            cmd::general::check_update,
            cmd::quotes::get_quotes,
            cmd::quotes::get_single_quote,
        ])
        .build(tauri_ctx)
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn run_backend(receiver: Receiver<i32>) {
    log::info!("Starting API server...");

    // `new_sidecar()` expects just the filename, NOT the whole path
    let t =
        TCommand::new_sidecar("main").expect("[Error] Failed to create `main.exe` binary command");
    let mut group = Command::from(t)
        .group_spawn()
        .expect("[Error] spawning api server process.");
    // If anyone knows how to log out stdout msg's from this process drop a comment. Rust is not my language.
    thread::spawn(move || loop {
        let s = receiver.recv();
        if s.unwrap() == -1 {
            group.kill().expect("[Error] killing api server process.");
        }
    });
}

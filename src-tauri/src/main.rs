// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(internal_output_capture)]

mod auto_start;
mod config;
mod logger;
mod os;
mod rpc;
mod service;
mod tls;

use anyhow::Result;
use std::{io::BufReader, process::Command};

use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;

#[cfg(target_os = "windows")]
use window_vibrancy::apply_mica;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use window_shadows::set_shadow;

use tauri::{
    generate_handler, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

use config::{
    get_proxy, has_password, input_password, set_password, set_proxy, test_proxy, try_password,
};
use logger::{clear_logs, get_logs};
use os::{is_win11, os_info};
use rpc::{run_rpc_server, stop_rpc_server};
use service::app::{get_build_info, get_server_url};

const MENU_ITEM_AUTO_START: &str = "Start at login";
const MENU_ITEM_QUIT: &str = "Quit";
const MENU_ITEM_ABOUT: &str = "About";
const MENUITEM_COPY_ADDR: &str = "Copy server address";
const MENUITEM_SHOW: &str = "Show window";

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main]
async fn main() {
    logger::setup_logger();
    log::info!("app started!");

    #[cfg(target_os = "linux")]
    tauri_plugin_deep_link::prepare("us.engy.focus-repro");

    #[cfg(not(target_os = "windows"))]
    tokio::spawn(async {
        let mut piped_stdout =
            <capture_stdio::PipedStdout as capture_stdio::Capture>::capture().unwrap();
        let mut buf_reader = BufReader::new(piped_stdout.get_reader());
        loop {
            let mut output = String::new();
            read_line_until_newline(&mut buf_reader, &mut output).unwrap();
            log::info!("{}", output);
        }
    });

    let show = CustomMenuItem::new(MENUITEM_SHOW, MENUITEM_SHOW);
    let copy_addr = CustomMenuItem::new(MENUITEM_COPY_ADDR, MENUITEM_COPY_ADDR);
    let quit = CustomMenuItem::new(MENU_ITEM_QUIT, MENU_ITEM_QUIT).accelerator("Cmd+Q");
    let auto_start = {
        let item = CustomMenuItem::new(MENU_ITEM_AUTO_START, MENU_ITEM_AUTO_START);
        match auto_start::AUTO_LAUNCH.as_ref() {
            Some(v) => {
                let enabled = v.is_enabled();
                if enabled.is_err() {
                    item.disabled()
                } else {
                    if enabled.unwrap() {
                        item.selected()
                    } else {
                        item
                    }
                }
            }
            None => item.disabled(),
        }
    };
    let about = CustomMenuItem::new(MENU_ITEM_ABOUT, MENU_ITEM_ABOUT);

    let system_tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(auto_start)
        .add_item(copy_addr)
        .add_item(about)
        .add_item(quit);
    let app = tauri::Builder::default()
        .invoke_handler(generate_handler![
            get_logs,
            clear_logs,
            stop_rpc_server,
            run_rpc_server,
            is_win11,
            os_info,
            set_proxy,
            get_proxy,
            test_proxy,
            get_server_url,
            get_build_info,
            has_password,
            input_password,
            set_password,
            try_password
        ])
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .setup(|app| {

            #[cfg(target_os="linux")]
            {
                let handle = app.handle();
                tauri_plugin_deep_link::register(
                    "aleoacc",
                    move |request| {
                        println!("Activated from second instance: {}", request);
                        if let Some(main_window) = handle.get_window("main") {
                            if let Err(err) = main_window.set_focus() {
                                eprintln!("Could not set focus on main window: {:?}", err);
                            }
                        }
                    },
                )
                .unwrap(/* If listening to the scheme is optional for your app, you don't want to unwrap here. */);
            }

            // hide dock icon on macOS
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            {
                let _ = window.set_decorations(true);
                apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            }

            #[cfg(target_os = "windows")]
            if is_win11() {
                apply_mica(&window, None)
                    .expect("Unsupported platform! 'apply_mica' is only supported on Windows11");
            }

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();

                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                MENUITEM_SHOW => {
                    let handle = app.app_handle();
                    if let Some(window) = handle.get_window("main") {
                        let _ = window.show();
                    };
                }
                MENU_ITEM_QUIT => {
                    println!("system tray received quit");
                    std::process::exit(0);
                }
                MENU_ITEM_ABOUT => {
                    let handle = app.app_handle();
                    tauri::WindowBuilder::new(
                        &handle,
                        "about",
                        tauri::WindowUrl::App("/about".into()),
                    )
                    .inner_size(400.0, 300.0)
                    .build()
                    .unwrap();
                }
                MENUITEM_COPY_ADDR => {
                    if let Ok(addr) = get_server_url() {
                        match ClipboardContext::new() {
                            Err(e) => {
                                log::error!("faied to get clipboard context: {}", e);
                            }
                            Ok(mut ctx) => {
                                if let Err(e) = ctx.set_contents(addr) {
                                    log::error!("faied to write clipboard: {}", e);
                                }
                            }
                        };
                    }
                }
                MENU_ITEM_AUTO_START => {
                    match auto_start::AUTO_LAUNCH.as_ref() {
                        Some(v) => {
                            let enabled = v.is_enabled();
                            if enabled.is_err() {
                                log::warn!(
                                    "failed to get auto start menu item enabled state: {}",
                                    enabled.as_ref().err().unwrap()
                                );
                                if let Err(e) = app
                                    .tray_handle()
                                    .get_item(MENU_ITEM_AUTO_START)
                                    .set_enabled(false)
                                {
                                    log::warn!(
                                        "failed to set auto start menu item to disabled: {}",
                                        e
                                    )
                                }
                            }

                            if enabled.is_ok() && enabled.unwrap() {
                                if let Err(e) = v.disable() {
                                    log::warn!("failed to disable auto start: {}", e);
                                } else {
                                    if let Err(e) = app
                                        .tray_handle()
                                        .get_item(MENU_ITEM_AUTO_START)
                                        .set_selected(false)
                                    {
                                        log::warn!(
                                            "failed to set auto start menu item to disabled: {}",
                                            e
                                        )
                                    }
                                }
                            } else {
                                if let Err(e) = v.enable() {
                                    log::warn!("failed to enable auto start: {}", e);
                                } else {
                                    if let Err(e) = app
                                        .tray_handle()
                                        .get_item(MENU_ITEM_AUTO_START)
                                        .set_selected(true)
                                    {
                                        log::warn!(
                                            "failed to set auto start menu item to disabled: {}",
                                            e
                                        )
                                    }
                                }
                            }
                        }
                        None => {}
                    };
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    //store app handle
    {
        let apphandle = app.app_handle();

        let mut handle = service::app::APP_HANDLE.lock().unwrap();

        *handle = Some(apphandle.clone());

        drop(handle);
    }

    #[cfg(target_os = "macos")]
    if let Ok(true) = is_another_instance_running(&app.config().tauri.bundle.identifier) {
        return;
    }

    _ = config::init();

    #[allow(unused)]
    app.run(|app, event| {});
}

#[cfg(target_os = "macos")]
fn is_another_instance_running(bundle_identifier: &str) -> Result<bool> {
    let output = Command::new("pgrep")
        .arg("-x")
        .arg("-f")
        .arg(&bundle_identifier)
        .output()?;

    Ok(output.status.success())
}

#[cfg(not(target_os = "windows"))]
fn read_line_until_newline(
    reader: &mut dyn std::io::BufRead,
    buf: &mut String,
) -> std::io::Result<()> {
    buf.clear();
    let mut byte_buf: [u8; 4] = [0; 4];

    loop {
        let num_bytes = reader.read(&mut byte_buf)?;
        if num_bytes == 0 {
            break;
        }

        let line = String::from_utf8_lossy(&byte_buf[..num_bytes]);
        if let Some(newline_pos) = line.find('\n') {
            buf.push_str(&line[..newline_pos]);
            break;
        }
        if let Some(newline_pos) = line.find('\r') {
            buf.push_str(&line[..newline_pos]);
            break;
        }
        buf.push_str(&line);
    }

    Ok(())
}

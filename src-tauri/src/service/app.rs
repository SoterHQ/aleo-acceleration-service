use anyhow::Result;
use serde::Serialize;
use std::{
    process::exit,
    sync::{Arc, Mutex},
};
use tauri::{api::dialog, Manager};

use lazy_static::lazy_static;

use crate::{
    config::{consts, Config},
    tls,
};

lazy_static! {
    pub static ref APP_HANDLE: Arc<Mutex<Option<tauri::AppHandle>>> = Arc::new(Mutex::new(None));
}

pub fn get_app_handle() -> tauri::AppHandle {
    let mu: std::sync::MutexGuard<'_, Option<tauri::AppHandle>> = APP_HANDLE.lock().unwrap();
    let handle: &tauri::AppHandle = mu.as_ref().unwrap();
    handle.clone()
}

#[tauri::command]
pub fn get_server_url() -> Result<String, String> {
    let fingerprint = match get_server_fingerprint() {
        Ok(v) => v,
        Err(e) => {
            let err_msg = format!("failed to get server fingerprint {:#?}", e);
            dialog::message(
                get_app_handle().get_window("main").as_ref(),
                "init error",
                err_msg,
            );
            return Err(e.to_string());
        }
    };

    let url = format!(
        "http://{}@{}:{}",
        fingerprint,
        "127.0.0.1",
        consts::RPC_PORT
    );

    Ok(url)
}

fn get_server_fingerprint() -> Result<String> {
    let secret = Config::get_config().get_secret_key()?;
    let public = tls::get_p256_pubkey(&secret);
    let fingerprint = tls::pubkey_to_fingerprint(&public);

    let fingerprint_str = hex::encode(fingerprint);
    Ok(fingerprint_str)
}

#[derive(Debug, Serialize)]
pub struct BuildInfo {
    pub time: String,
    pub commit: String,
}

#[tauri::command]
pub fn get_build_info() -> BuildInfo {
    let commit = env!("git_commit").to_string();
    let time = env!("build_time").to_string();

    BuildInfo { time, commit }
}

pub fn error_dialog(message: &str) {
    dialog::blocking::message(
        get_app_handle().get_window("main").as_ref(),
        "error",
        message,
    );
    exit(1)
}

pub fn update_dialog(version: &str) {
    if dialog::blocking::confirm(
        get_app_handle().get_window("main").as_ref(),
        "Aleo acc update",
        format!(
            "version {} is required, current version is {}, click OK to download newer version",
            version,
            env!("CARGO_PKG_VERSION")
        ),
    ) {
        let _ = tauri::api::shell::open(
            &get_app_handle().shell_scope(),
            "https://github.com/SoterHQ/aleo-acceleration-service/releases",
            None,
        );
    }
}

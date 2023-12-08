use std::sync::Mutex;

use chrono::Local;
use lazy_static::lazy_static;
use log::{Log, Metadata, Record};

lazy_static! {
    static ref LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

struct MemoryLogger {
    max_lines: usize,
}

impl MemoryLogger {
    fn new(max_lines: usize) -> Self {
        MemoryLogger { max_lines }
    }
}

impl Log for MemoryLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let current_time = Local::now();
        let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!(
            "{} :: {} - {}",
            formatted_time,
            record.level(),
            record.args()
        );
        // println!("{}", log_entry);
        let mut logs = LOGS.lock().unwrap();
        logs.push(log_entry);
        if logs.len() > self.max_lines {
            let excess_lines = logs.len() - self.max_lines;
            logs.drain(0..excess_lines);
        }
    }

    fn flush(&self) {}
}

pub fn setup_logger() {
    let max_lines = 1000;
    let logger = Box::new(MemoryLogger::new(max_lines));
    log::set_boxed_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("logger init");
}

#[tauri::command]
pub fn get_logs() -> Vec<String> {
    let logs = LOGS.lock().unwrap();
    logs.to_vec()
}

#[tauri::command]
pub fn clear_logs() {
    let mut logs = LOGS.lock().unwrap();
    logs.clear();
}

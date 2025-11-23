use slint::invoke_from_event_loop;

use crate::{generated::DialogSeverity, ui::dialogs::present_dialog_standalone};

static BACKEND_URL: &str = env!("BACKEND_URL");

pub fn safe<F>(f: F) where F: FnOnce() + Send + 'static {
    let _ = invoke_from_event_loop(f);
}

pub fn open_discord() {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord").as_str()) {
        present_dialog_standalone("System Error", format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

pub fn open_youtube() {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord").as_str()) {
        present_dialog_standalone("System Error", format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

pub fn open_x() {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord").as_str()) {
        present_dialog_standalone("System Error", format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

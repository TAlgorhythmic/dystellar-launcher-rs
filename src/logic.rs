use slint::invoke_from_event_loop;

use crate::{generated::{DialogSeverity, Main}, ui::dialogs::present_dialog};

static BACKEND_URL: &str = env!("BACKEND_URL");

pub fn safe<F>(f: F) where F: FnOnce() + Send + 'static {
    invoke_from_event_loop(f);
}

pub fn open_discord(ui: &Main) {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord")) {
        present_dialog(ui, format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

pub fn open_youtube(ui: &Main) {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord")) {
        present_dialog(ui, format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

pub fn open_x(ui: &Main) {
    if let Err(e) = webbrowser::open(format!("{BACKEND_URL}/discord")) {
        present_dialog(ui, format!("Failed to open browser: {}", e.to_string()).as_str(), DialogSeverity::Error);
    }
}

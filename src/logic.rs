static BACKEND_URL: &str = env!("BACKEND_URL");

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

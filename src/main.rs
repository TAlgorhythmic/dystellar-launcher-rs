#![windows_subsystem = "windows"]

use std::error::Error;

use slint::ComponentHandle;

use crate::generated::FallbackDialog;

mod ui;
mod api;
mod logic;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/main_ui.rs"));
    include!(concat!(env!("OUT_DIR"), "/welcome_ui.rs"));
    include!(concat!(env!("OUT_DIR"), "/fallback_dialog_ui.rs"));
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let res = ui::launcher::run();
    
    if let Err(err) = res {
        let dialog = FallbackDialog::new()?;

        dialog.set_text(format!("Failed to initialize: {}", err.to_string()).into());
        dialog.on_close({
            let dialog = dialog.clone_strong();
            move || { dialog.hide(); }
        });

        dialog.run()?;
    }

    Ok(())
}

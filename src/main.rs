#![windows_subsystem = "windows"]

use std::error::Error;

use slint::ComponentHandle;

use crate::generated::{DialogData, DialogSeverity, DialogType, FallbackDialog};

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

        dialog.set_dialog_data(DialogData {
            content: format!("Failed to initialize: {}", err.to_string()).into(),
            r#type: DialogType::Basic,
            severity: DialogSeverity::Error
        });
        dialog.set_name("Platform Error".into());
        dialog.on_close({
            let dialog = dialog.clone_strong();
            move || { let _ = dialog.hide(); }
        });

        dialog.run()?;
    }

    Ok(())
}

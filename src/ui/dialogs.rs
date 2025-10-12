use slint::ComponentHandle;

use crate::generated::{DialogData, DialogSeverity, DialogType, FallbackDialog, Main};

pub fn present_dialog(ui: &Main, text: &str, severity: DialogSeverity) {
    ui.set_dialog_data(DialogData { content: text.into(), severity, shown: true, r#type: DialogType::Basic });
    ui.invoke_show_popup();
}

pub fn present_confirmation_dialog<F>(ui: &Main, text: &str, severity: DialogSeverity, exec: F)
where F: Fn() + Send + Sync {
    ui.set_dialog_data(DialogData { content: text.into(), severity, shown: true, r#type: DialogType::Basic });
    ui.on_dialog_confirmed(exec);
    ui.invoke_show_popup();
}

pub fn present_dialog_standalone(title: &str, text: &str) {
    let ui = FallbackDialog::new().unwrap();

    ui.set_text(text.into());
    ui.set_name(title.into());
    ui.set_type(DialogType::Basic);
    ui.on_close(|| ui.hide());
    ui.show();
}

pub fn present_confirmation_dialog_standalone<F>(title: &str, text: &str, exec: F)
where F: Fn() + Send + Sync {
    let ui = FallbackDialog::new().unwrap();

    ui.set_type(DialogType::Confirmation);
    ui.set_name(title.into());
    ui.set_text(text.into());
    ui.on_close(|| ui.hide());
    ui.on_confirm(exec);
    ui.show();
}

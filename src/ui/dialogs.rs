
use crate::ui::launcher::{DialogSeverity, DialogType, Main};
use super::launcher::DialogData;

pub fn present_dialog(ui: &Main, title: &str, text: &str, severity: DialogSeverity) {
    ui.set_dialog_data(DialogData { content: text.into(), severity, shown: true, title: title.into(), r#type: DialogType::Basic });
}

pub fn present_confirmation_dialog<F>(ui: &Main, title: &str, text: &str, severity: DialogSeverity, exec: F)
where F: Fn() + Send + Sync {
    ui.set_dialog_data(DialogData { content: text.into(), severity, shown: true, title: title.into(), r#type: DialogType::Basic });
}

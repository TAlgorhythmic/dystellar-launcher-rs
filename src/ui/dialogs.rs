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

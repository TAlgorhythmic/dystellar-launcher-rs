use slint::ComponentHandle;

use crate::generated::{DialogData, DialogSeverity, DialogType, FallbackDialog};

pub fn present_dialog_standalone(title: &str, text: &str, severity: DialogSeverity) {
    let ui = FallbackDialog::new().unwrap();

    ui.set_name(title.into());
    ui.set_dialog_data(DialogData { content: text.into(), severity, r#type: DialogType::Basic });
    ui.on_close({
        let ui = ui.clone_strong();
        move || { let _ = ui.hide(); }
    });
    let _ = ui.show();
}

pub fn present_confirmation_dialog_standalone<F>(title: &str, text: &str, severity: DialogSeverity, exec: F)
where F: Fn() + Send + Sync + 'static {
    let ui = FallbackDialog::new().unwrap();

    ui.set_name(title.into());
    ui.set_dialog_data(DialogData { content: text.into(), severity, r#type: DialogType::Confirmation });
    ui.on_close({
        let ui = ui.clone_strong();
        move || { let _ = ui.hide(); }
    });
    ui.on_confirm({
        let ui = ui.clone_strong();
        move || {
            exec();
            let _ = ui.hide();
        }
    });
    let _ = ui.show();
}

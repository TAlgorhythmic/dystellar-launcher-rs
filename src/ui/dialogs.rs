use std::{error::Error, sync::{Arc, Mutex}};

use slint::ComponentHandle;

use crate::{api::{control::{database::store_session, http::login}, typedef::ms_session::MicrosoftSession}, generated::{DialogData, DialogSeverity, DialogType, FallbackDialog, WelcomeUI}};

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

pub fn create_welcome_ui(session_mutex: Arc<Mutex<Option<MicrosoftSession>>>) -> Result<WelcomeUI, Box<dyn Error>> {
    let win = WelcomeUI::new()?;
    let win_weak = win.as_weak();

    win.on_login(move || {
        let win = win_weak.upgrade().unwrap();
        let win_weak = win.as_weak();
        let mutex_cl = session_mutex.clone();

        win.set_waiting(true);
        login(move |result| {
            let win = win_weak.upgrade().unwrap();

            win.set_waiting(false);
            if let Err(err) = &result {
                present_dialog_standalone(err.title, &err.description, DialogSeverity::Error);
                return;
            }

            let session = result.unwrap();
            if let Err(err) = store_session(&session.access_token, &session.refresh_token) {
                present_dialog_standalone("Failed to store session", format!("Failed to store session in storage: {} You'll have to login again next time.", err.to_string()).as_str(), DialogSeverity::Error);
            }

            let mut guard = mutex_cl.lock().unwrap();

            *guard = Some(session);
            let _ = win.hide();
        });
    });

    Ok(win)
}

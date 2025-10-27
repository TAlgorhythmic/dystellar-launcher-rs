use crate::generated::{AppState, Callbacks, DialogSeverity, Main, TasksGroup, WelcomeUI};
use crate::logic::{open_discord, open_youtube};
use crate::ui::dialogs::{present_dialog, present_dialog_standalone};
use crate::{api::control::database::store_session, logic::open_x};
use crate::api::control::http::{login, login_existing};
use slint::{ComponentHandle, Model, ModelRc, VecModel, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::error::Error;

fn setup_callbacks(ui: Weak<Main>, session: Arc<Mutex<Option<MicrosoftSession>>>) {
    let ui_strong = ui.upgrade().unwrap();
    let callbacks = ui_strong.global::<Callbacks>();

    callbacks.on_click_x({
        let ui = ui.clone();
        move || open_x(&ui.upgrade().unwrap())
    });
    callbacks.on_click_discord({
        let ui = ui.clone();
        move || open_discord(&ui.upgrade().unwrap())
    });
    callbacks.on_click_youtube({
        let ui = ui.clone();
        move || open_youtube(&ui.upgrade().unwrap())
    });
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let tokens = retrieve_session()?;
    let s_mutex: Arc<Mutex<Option<MicrosoftSession>>> = Arc::new(Mutex::new(None));

    if tokens.is_none() {
        let win = WelcomeUI::new()?;
        let mutex_cl = s_mutex.clone();
        let win_weak = win.as_weak();

        win.on_login(move || {
            let win = win_weak.upgrade().unwrap();
            let win_weak = win.as_weak();
            let mutex_cl = mutex_cl.clone();

            win.set_waiting(true);
            login(move |result| {
                let win = win_weak.upgrade().unwrap();

                win.set_waiting(false);
                if let Err(err) = &result {
                    present_dialog_standalone(err.title, &err.description);
                    return;
                }

                let session = result.unwrap();
                if let Err(err) = store_session(&session.access_token, &session.refresh_token) {
                    present_dialog_standalone("Failed to store session", format!("Failed to store session in storage: {} You'll have to login again next time.", err.to_string()).as_str());
                }

                let mut guard = mutex_cl.lock().unwrap();

                *guard = Some(session);
                let _ = win.hide();
            });
        });

        win.run()?; // Blocking call until the user is logged in.

        // If the user closes the program without logging in.
        let session = s_mutex.lock().unwrap();
        if session.is_none() {
            return Ok(());
        }
        drop(session);
    }
    let session = s_mutex.lock().unwrap();
    let ui = Main::new()?;

    setup_callbacks(ui.as_weak(), s_mutex.clone());
    ui.set_groups(ModelRc::from(Rc::new(VecModel::from(vec![]))));

    if session.is_none() {
        let (access_token, refresh_token) = tokens.unwrap();
        let ui_weak = ui.as_weak();
        let mutex_cl = s_mutex.clone();

        login_existing(access_token, refresh_token, move |result| {
            if let Err(err) = &result {
                present_dialog(&ui_weak.upgrade().unwrap(), &err.to_string(), DialogSeverity::Error);
                return;
            }

            let session = result.unwrap();

            if let Err(err) = store_session(&session.access_token, &session.refresh_token) {
                present_dialog_standalone("Failed to store session", format!("Failed to store session in storage: {} You'll have to login again next time.", err.to_string()).as_str());
            }

            let mut guard = mutex_cl.lock().unwrap();
            let ui = ui_weak.upgrade().unwrap();

            *guard = Some(session);
            ui.set_app_state(AppState::Ready);
        });
    }
    drop(session);
    ui.run()?;
    Ok(())
}

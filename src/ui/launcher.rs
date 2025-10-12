use crate::generated::{AppState, Callbacks, Main, WelcomeUI};
use crate::logic::{open_discord, open_youtube};
use crate::{api::control::database::store_session, logic::open_x};
use crate::api::control::http::{login, login_existing};
use slint::{ComponentHandle, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}};
use std::sync::{Arc, Mutex};
use std::error::Error;

fn setup_callbacks(ui: Weak<Main>, session: Arc<Mutex<Option<MicrosoftSession>>>) {
    let ui = ui.upgrade().unwrap();
    let callbacks = ui.global::<Callbacks>();

    callbacks.on_click_x(|| open_x(&ui));
    callbacks.on_click_discord(|| open_discord(&ui));
    callbacks.on_click_youtube(|| open_youtube(&ui));
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let tokens = retrieve_session()?;
    let s_mutex: Arc<Mutex<Option<MicrosoftSession>>> = Arc::new(Mutex::new(None));

    if tokens.is_none() {
        let win = WelcomeUI::new()?;

        win.on_login(|| {
            win.set_waiting(true);
            login(|session| {
                store_session(&session.access_token, &session.refresh_token);

                let mut guard = s_mutex.lock().unwrap();

                *guard = Some(session);
                win.set_waiting(false);
                win.hide();
            });
        });

        win.run()?;
    }

    let session = s_mutex.lock()?;
    let ui = Main::new()?;

    if session.is_none() {
        let (access_token, refresh_token) = tokens.unwrap();

        login_existing(ui.as_weak(), access_token, refresh_token, |session| {
            store_session(&session.access_token, &session.refresh_token);

            let mut guard = s_mutex.lock().unwrap();
            *guard = Some(session);
            ui.set_app_state(AppState::Ready);
        });
    }
    drop(session);
    setup_callbacks(ui.as_weak(), s_mutex);

    ui.run()?;
    Ok(())
}

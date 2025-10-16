use crate::generated::{AppState, Callbacks, Main, TasksGroup, WelcomeUI};
use crate::logic::{open_discord, open_youtube};
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

        win.run()?; // Blocking call until the user is logged in.
    }

    let session = s_mutex.lock()?;
    let ui = Main::new()?;

    setup_callbacks(ui.as_weak(), s_mutex);
    ui.set_groups(ModelRc::from(Rc::new(VecModel::from(vec![]))));
    let i = ui.get_groups().row_data(0).unwrap();

    if session.is_none() {
        let (access_token, refresh_token) = tokens.unwrap();

        login_existing(ui.as_weak(), access_token, refresh_token, move |session| {
            store_session(&session.access_token, &session.refresh_token);

            let mut guard = s_mutex.lock().unwrap();
            *guard = Some(session);
            ui.set_app_state(AppState::Ready);
        });
    }
    drop(session);
    ui.run()?;
    Ok(())
}

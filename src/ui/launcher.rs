use crate::generated::{Callbacks, Main, WelcomeUI};
use crate::logic::{open_discord, open_youtube};
use crate::ui::app::new_welcome_window;
use crate::{api::control::database::store_session, logic::open_x};
use crate::api::control::http::login_existing;
use slint::{ComponentHandle, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}};
use std::sync::{Arc, Mutex};
use std::{cell::RefCell, error::Error};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

fn setup_callbacks(ui: Weak<Main>) {
    let ui = ui.upgrade().unwrap();
    let callbacks = ui.global::<Callbacks>();

    callbacks.on_click_x(|| open_x(&ui));
    callbacks.on_click_discord(|| open_discord(&ui));
    callbacks.on_click_youtube(|| open_youtube(&ui));
}

pub fn present_main_ui() -> Result<Main, slint::PlatformError> {
    let ui = Main::new()?;

    setup_callbacks(ui.as_weak());
    ui.run()?;

    Ok(ui)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let tokens = retrieve_session();
    let s_mutex: Arc<Mutex<Option<MicrosoftSession>>> = Arc::new(Mutex::new(None));

    if tokens.is_none() {
        new_welcome_window(s_mutex.clone())?.run()?;
    }

    let session = s_mutex.lock()?;
    let ui = Main::new()
    if session.is_none() {
        let (access_token, refresh_token) = tokens.unwrap();

        login_existing(access_token, refresh_token)?;
    }

    if let Err(err) = store_session(tokens.get_access_token(), tokens.get_refresh_token()) {
        show_dialog("Failed to save session", format!("An unexpected error occurred when updating database: {} Please contact support.", err.to_string()).as_str(), None, ICON_ERROR);
        return;
    }

    SESSION.with(|s| s.replace(Some(tokens)));

}

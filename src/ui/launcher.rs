use slint::{ComponentHandle, Weak};

use crate::{api::{control::database::retrieve_session, typedef::ms_session::MicrosoftSession}, ui::components::open_link_browser};
use std::{cell::RefCell, error::Error};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

slint::include_modules!();

thread_local! {
    pub static SESSION: RefCell<Option<MicrosoftSession>> = RefCell::new(None);
}

fn setup_callbacks(ui: Weak<Main>) {
    let ui = ui.upgrade().unwrap();
    let callbacks = ui.global::<Callbacks>();

    callbacks.on_click_x(|| open_link_browser("asda"));
}

pub fn present_main_ui() -> Result<Main, slint::PlatformError> {
    let ui = Main::new()?;

    setup_callbacks(ui.as_weak());
    ui.run()?;

    Ok(ui)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let session = retrieve_session().expect("FATAL: Failed to retrieve session, unable to continue");
    if session.is_none() {
        let welcome_screen = welcome_login_screen(&app);

        welcome_screen.present();
    } else {
        let (access_token, refresh_token) = session.unwrap();
        let ui = present_main_ui(&app);

        let session_opt = login_existing(&access_token, &refresh_token);
        if let Err(err) = &session_opt {
            show_dialog("Failed to refresh tokens", format!("An unexpected error occurred when fetching tokens: {}", err.to_string()).as_str(), None, ICON_ERROR);
            return;
        }
        
        let tokens = session_opt.unwrap();
        if let Err(err) = store_session(tokens.get_access_token(), tokens.get_refresh_token()) {
            show_dialog("Failed to save session", format!("An unexpected error occurred when updating database: {} Please contact support.", err.to_string()).as_str(), None, ICON_ERROR);
            return;
        }

        SESSION.with(|s| s.replace(Some(tokens)));

    }

    present_main_ui()
}

use std::error::Error;

use crate::{api::control::{database::store_session, http::login}, generated::WelcomeUI};

pub fn new_welcome_window() -> Result<WelcomeUI, Box<dyn Error>> {
    let win = WelcomeUI::new()?;

    win.on_login(|| {
        win.set_waiting(true);
        login(|session| {
            store_session(&session.access_token, &session.refresh_token);
        });
    });
}

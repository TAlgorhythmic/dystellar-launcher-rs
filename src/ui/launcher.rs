use crate::api::control::{database::{retrieve_session, store_session}, http::login_existing};
use crate::api::typedef::ms_session::MicrosoftSession;
use crate::css;
use crate::ui::components::{show_dialog, ICON_ERROR};
use crate::ui::main_ui::init_main_ui;
use crate::ui::welcome_ui::welcome_login_screen;
use std::cell::RefCell;

use gtk::prelude::*;
use gtk::glib;
use gtk::Box;
use libadwaita::HeaderBar;
use libadwaita::{Application, ApplicationWindow};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

thread_local! {
    pub static APP_INSTANCE: RefCell<Option<Application>> = RefCell::new(None);
    pub static SESSION: RefCell<Option<MicrosoftSession>> = RefCell::new(None);
}

pub fn present_main_ui(app: &Application) {
    let ui = init_main_ui();

    let parent = Box::builder().halign(gtk::Align::Fill).valign(gtk::Align::Fill).orientation(gtk::Orientation::Vertical).build();
    let header = HeaderBar::builder()
        .css_classes(["header"])
        .show_end_title_buttons(true)
        .build();

    parent.append(&header);
    parent.append(&ui.main_content);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dystellar Network MMORPG | Official Launcher")
        .name("Dystellar Network MMORPG | Official Launcher")
        .default_width(1280)
        .default_height(720)
        .content(&parent)
        .decorated(true)
        .css_classes(["window"])
        .build();

    window.present();
}

pub fn init(app: &Application) {
    css::inject_css();

    let session = retrieve_session().expect("FATAL: Failed to retrieve session, unable to continue");
    if session.is_none() {
        let welcome_screen = welcome_login_screen(&app);

        welcome_screen.present();
    } else {
        let (access_token, refresh_token) = session.unwrap();

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

        present_main_ui(&app);
    }
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    APP_INSTANCE.replace(Some(app.clone()));
    app.connect_activate(init);
    
    app.run()
}

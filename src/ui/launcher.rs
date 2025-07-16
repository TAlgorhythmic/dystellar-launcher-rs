use crate::css;
use std::cell::OnceCell;

use crate::ui::main_ui::init_main_ui;
use gtk::prelude::*;
use gtk::glib;
use libadwaita::{Application, ApplicationWindow};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

thread_local! {
    pub static APP_INSTANCE: OnceCell<Application> = OnceCell::new();
}

pub fn init(app: &Application) {
    css::inject_css();

    let ui = init_main_ui();
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dystellar Network MMORPG | Official Launcher")
        .name("Dystellar Network MMORPG | Official Launcher")
        .default_width(1280)
        .default_height(720)
        .content(&ui.main_content)
        .css_classes(["window"])
        .build();

    window.set_child(Some(&ui.main_content));
    window.present();
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    APP_INSTANCE.with(|cell| cell.set(app.clone()).expect("Only assign once"));
    app.connect_activate(init);
    

    app.run()
}

use crate::css;
use crate::ui::main_ui::init_main_ui;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

pub fn init(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dystellar Network MMORPG | Official Launcher")
        .name("Dystellar Network MMORPG | Official Launcher")
        .default_width(1280)
        .default_height(720)
        .decorated(true)
        .css_classes(["window"])
        .build();

    css::inject_css();

    let ui = init_main_ui();

    window.set_child(Some(&ui.main_content));
    window.present();
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(init);

    app.run()
}

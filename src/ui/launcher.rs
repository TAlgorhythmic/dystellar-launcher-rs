use crate::ui::components;
use crate::css;
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
        .build();

    css::inject_css();

    window.style_context().add_class("window");
    window.set_decorated(true);

    window.set_child(Some(&components::MAIN_UI.main_content));
   
    window.present();
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(init);

    app.run()
}

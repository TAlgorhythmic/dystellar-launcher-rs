use std::cell::OnceCell;

use crate::css;
use crate::ui::components::{show_confirmation_dialog, ICON_INFO};
use crate::ui::main_ui::init_main_ui;
use gtk::{prelude::*, Settings};
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

thread_local! {
    pub static APP_INSTANCE: OnceCell<Application> = OnceCell::new();
}

pub fn force_enable_animations() {
    if let Some(settings) = Settings::default() {
        settings.set_gtk_enable_animations(true);
    }
}

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
    let _ = gtk::init();
    #[cfg(target_os = "windows")]
    force_enable_animations();
    let app = Application::builder().application_id(APP_ID).build();
    APP_INSTANCE.with(|cell| cell.set(app.clone()).expect("Only assign once"));
    app.connect_activate(init);
    

    app.run()
}

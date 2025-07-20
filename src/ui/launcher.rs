use crate::css;
use std::cell::OnceCell;

use crate::ui::main_ui::init_main_ui;
use gtk::prelude::*;
use gtk::glib;
use gtk::Box;
use libadwaita::HeaderBar;
use libadwaita::{Application, ApplicationWindow};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

thread_local! {
    pub static APP_INSTANCE: OnceCell<Application> = OnceCell::new();
}

pub fn init(app: &Application) {
    css::inject_css();

    let ui = init_main_ui();
    let parent = Box::builder().halign(gtk::Align::Fill).valign(gtk::Align::Fill).orientation(gtk::Orientation::Vertical).build();

    parent.append(&HeaderBar::builder().css_classes(["header"]).build());
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

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    APP_INSTANCE.with(|cell| cell.set(app.clone()).expect("Only assign once"));
    app.connect_activate(init);
    

    app.run()
}

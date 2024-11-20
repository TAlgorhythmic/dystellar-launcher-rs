use crate::ui::components;
use crate::css;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "gg.dystellar.mmorpg.Launcher";

fn load_font(data: &[u8]) {
    let map = gtk::pango::FontMap
}

pub fn init(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dystellar Network MMORPG | Official Launcher")
        .name("Dystellar Network MMORPG | Official Launcher")
        .default_width(1280)
        .default_height(720)
        .build();

    css::inject_css();

    #[cfg(target_os = "windows")]
        let f_bytes: &[u8] = include_bytes!(".\\..\\..\\assets\\fonts\\rajdhani.ttf");
    #[cfg(not(target_os = "windows"))]
        let f_bytes: &[u8] = include_bytes!("./../../assets/fonts/rajdhani.ttf");

    

    window.style_context().add_class("window");
    window.set_decorated(true);

    unsafe {
        window.set_child(Some(&components::MAIN_UI.main_content));
    }
   
    window.present();
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(init);

    app.run()
}

use crate::ui::components;
use crate::ui::icons;
use crate::css;
use gtk::prelude::*;
use gtk::{gdk_pixbuf, Application, ApplicationWindow, glib};

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
    window.set_decorated(false);

    components::init_components(&window);

    window.present();
}

pub fn run() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(init);

    app.run()
}

fn init_icons() -> Box<[gtk::Image]> {
    let loader_x = gdk_pixbuf::PixbufLoader::new();
    let loader_d = gdk_pixbuf::PixbufLoader::new();
    let loader_y = gdk_pixbuf::PixbufLoader::new();

    loader_x.write(icons::X.as_bytes()).expect("Error loading X icon.");
    loader_d.write(icons::DISCORD.as_bytes()).expect("Error loading Discord icon.");
    loader_y.write(icons::YOUTUBE.as_bytes()).expect("Error loading Youtube icon.");

    let pixbuf_x = loader_x.pixbuf().expect("Error. (X)");
    let pixbuf_d = loader_d.pixbuf().expect("Error. (Discord)");
    let pixbuf_y = loader_y.pixbuf().expect("Error. (Youtube)");

    let img_x = gtk::Image::new(); img_x.set_from_pixbuf(Some(&pixbuf_x));
    let img_d = gtk::Image::new(); img_d.set_from_pixbuf(Some(&pixbuf_d));
    let img_y = gtk::Image::new(); img_y.set_from_pixbuf(Some(&pixbuf_y));

    Box::new([img_x, img_d, img_y])
}

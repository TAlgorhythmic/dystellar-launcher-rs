use gtk::{Image, Window};

fn init_dialog(title: &str) -> Window {
    Window::builder()
        .title(title)
        .default_width(800)
        .default_height(300)
        .css_classes(["dialog"])
        .resizable(false)
        .name(title)
        .modal(true)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .hexpand(false)
        .vexpand(true)
        .decorated(true)
        .visible(true)
        .build()
}

pub fn init_confirmation_dialog<F>(title: &str, message: &str, icon: Image, f: F) -> Window
where
    F: FnOnce() -> (),
{
    let window = init_dialog(title);
    let child = 
}

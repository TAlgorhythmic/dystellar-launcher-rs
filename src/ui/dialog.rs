use gtk::prelude::*;
use gtk::{Box, Button, Image, Label, Window};

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

fn build_label(msg: &str) -> Label {
    Label::builder()
        .wrap(true)
        .hexpand(false)
        .vexpand(true)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Start)
        .label(msg)
        .build()
}

fn build_child() -> Box {
    Box::builder()
        .hexpand(false)
        .vexpand(true)
        .css_classes(["dialog-container"])
        .spacing(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Vertical)
        .build()
}

fn build_dialog_content(msg: &str, icon: &Image) -> Box {
    let res = Box::builder()
        .hexpand(false)
        .vexpand(true)
        .css_classes(["dialog-content"])
        .spacing(10)
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Start)
        .halign(gtk::Align::Center)
        .build();

    icon.set_vexpand(false);
    icon.set_hexpand(false);
    icon.set_valign(gtk::Align::Center);
    icon.set_halign(gtk::Align::Start);
    icon.set_css_classes(&["dialog-icon"]);

    let label = build_label(msg);
    
    res.append(icon);
    res.append(&label);
    res
}

pub fn init_confirmation_dialog<F>(title: &str, message: &str, icon: &Image, f: F) -> Window
where
    F: Fn() -> () + 'static
{
    let window = init_dialog(title);
    let child = build_child();

    child.append(&build_dialog_content(message, icon));

    let options = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .vexpand(false)
        .hexpand(false)
        .spacing(20)
        .homogeneous(true)
        .build();

    let okbutton = Button::builder()
        .css_classes(["dialog-ok-btn"])
        .label("Proceed")
        .focusable(true)
        .build();
    let cancelbutton = Button::builder()
        .css_classes(["dialog-cancel-btn"])
        .label("Cancel")
        .build();

    let wc = window.clone();
    let wc2 = window.clone();

    cancelbutton.connect_clicked(move |_| wc.close());

    okbutton.connect_clicked(move |_| {
        wc2.clone();
        f();
    });

    options.append(&cancelbutton);
    options.append(&okbutton);

    window.set_focus_child(Some(&okbutton));
    window
}

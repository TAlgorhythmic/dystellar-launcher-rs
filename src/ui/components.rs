use gtk::{ApplicationWindow, Button, Label, Box, Image, MenuButton, Popover};
use gtk::prelude::*;

pub fn init_components(window: &ApplicationWindow) {
    window.set_child(Some(&main_box()));
}

fn btn_play() -> Button {
    let btn: Button = Button::builder()
        .can_focus(false)
        .name("Launch")
        .label("Launch")
        .build();

    btn.add_css_class("launch-btn");

    btn
}

fn btn_regular(label: &str) -> Button {
    let btn = Button::with_label(label);
    btn.add_css_class("regular-btn");

    btn
}

fn info_comp(name: &str) -> Box {
    Box::builder()
        .can_focus(false)
        .name(name)
        .css_name("box")
        .build()
}

fn main_box() -> Box {
    let div = Box::builder()
        .can_focus(false)
        .orientation(gtk::Orientation::Horizontal)
        .css_name("main-content")
        .build();

    div.append(&btn_play());

    div
}

fn generic_label(text: &str) -> Label {
    let label = Label::builder()
        .can_focus(false)
        .label(text)
        .build();

    label.add_css_class("generic-label");

    label
}

fn btn_container() -> Box {
    let container = Box::builder()
        .can_focus(false)
        .build();

    container.add_css_class("btn-container");

    container
}

fn inner_button_label(label: &str) -> Button {
    let ib = Button::with_label(label);
    
    ib.add_css_class("inner-button");
    ib
}

fn inner_button_img(img: Image) -> Button {
    let btn = Button::new();
    
    btn.set_child(Some(&img));
    btn.add_css_class("inner-button");

    btn
}

fn inner_button_both(label: &str, img: Image) -> Button {
    let btn = Button::new();
    
    let cont = Box::new(gtk::Orientation::Horizontal, 2);
    cont.append(&img);
    cont.append(&Label::new(Some(label)));
    cont.add_css_class("inner-box");

    btn.set_child(Some(&cont));

    btn
}

fn acc_manager() -> MenuButton {
    let dpdn = MenuButton::new();
    dpdn.set_icon_name("asas");

    let options = Popover::new();
    
    dpdn.set_popover(Some(&options));

    dpdn
}

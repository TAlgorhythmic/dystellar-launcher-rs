use gtk::{ApplicationWindow, Button, Label, Box, Image, MenuButton, Popover, Grid};
use gtk::prelude::*;
use crate::ui;

pub fn init_components(window: &ApplicationWindow) {
    // TODO: do better
    window.set_child(Some(&main_box()));
}

pub fn btn_play() -> Button {
    let btn: Button = Button::builder()
        .can_focus(false)
        .name("Launch")
        .label("Launch")
        .build();

    btn.add_css_class("launch-btn");

    btn
}

pub fn btn_regular(label: &str) -> Button {
    let btn = Button::with_label(label);
    btn.add_css_class("regular-btn");

    btn
}

pub fn info_comp(name: &str) -> Box {
    Box::builder()
        .can_focus(false)
        .name(name)
        .css_name("box")
        .build()
}

pub fn main_box() -> Grid {
    let div = Grid::builder()
        .can_focus(false)
        .orientation(gtk::Orientation::Horizontal)
        .css_name("main-content")
        .build();

    div
}

pub fn generic_label(text: &str) -> Label {
    let label = Label::builder()
        .can_focus(false)
        .label(text)
        .build();

    label.add_css_class("generic-label");

    label
}

pub fn btn_container() -> Box {
    let container = Box::builder()
        .can_focus(false)
        .build();

    container.add_css_class("btn-container");

    container
}

pub fn inner_button_label(label: &str) -> Button {
    let ib = Button::with_label(label);
    
    ib.add_css_class("inner-button");
    ib
}

pub fn inner_button_img(img: Image) -> Button {
    let btn = Button::new();
    
    btn.set_child(Some(&img));
    btn.add_css_class("inner-button");

    btn
}

pub fn inner_button_both(label: &str, img: Image) -> Button {
    let btn = Button::new();
    
    let cont = Box::new(gtk::Orientation::Horizontal, 2);
    cont.append(&img);
    cont.append(&Label::new(Some(label)));
    cont.add_css_class("inner-box");

    btn.set_child(Some(&cont));

    btn
}

pub fn acc_manager() -> MenuButton {
    let dpdn = MenuButton::new();

    let options = Popover::new();
    let opts = Box::new(gtk::Orientation::Vertical, 2);
    opts.append(&inner_button_label("Log in"));
    options.set_child(Some(&opts));
    
    dpdn.set_popover(Some(&options));

    dpdn
}

use gtk::{ApplicationWindow, Button, Label, Box};
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

fn inner_button1(label: &str) -> Button {
    let ib = Button::with_label(label);
    
    ib.add_css_class("inner-button1");
    ib
}

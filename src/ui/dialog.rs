use gtk::{Box, Button, Image, Label};
use libadwaita::{prelude::*, Dialog, HeaderBar};

use crate::ui::animations::add_clickable_animation_btn;

fn init_dialog(title: &str) -> Dialog {
    Dialog::builder()
        .title(title)
        .css_classes(["dialog"])
        .presentation_mode(libadwaita::DialogPresentationMode::Floating)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Center)
        .overflow(gtk::Overflow::Hidden)
        .hexpand(true)
        .vexpand(true)
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
        .halign(gtk::Align::Center)
        .label(msg)
        .build()
}

fn build_child() -> Box {
    Box::builder()
        .hexpand(true)
        .vexpand(true)
        .css_classes(["dialog-container"])
        .spacing(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Fill)
        .orientation(gtk::Orientation::Vertical)
        .build()
}

fn build_dialog_content(msg: &str, icon: &Image) -> Box {
    let res = Box::builder()
        .hexpand(true)
        .vexpand(true)
        .css_classes(["dialog-content"])
        .spacing(10)
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Start)
        .halign(gtk::Align::Start)
        .build();

    let inner = Box::builder()
        .hexpand(true)
        .vexpand(true)
        .orientation(gtk::Orientation::Horizontal)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Start)
        .build();

    icon.set_vexpand(false);
    icon.set_hexpand(false);
    icon.set_valign(gtk::Align::Center);
    icon.set_halign(gtk::Align::Start);
    icon.set_css_classes(&["dialog-icon"]);

    let label = build_label(msg);
    
    res.append(icon);
    inner.append(&label);
    res.append(&inner);
    res
}

pub fn init_confirmation_dialog<F>(title: &str, message: &str, icon: &Image, f: F, ok_btn_label: Option<&str>) -> Dialog
where
    F: Fn() -> () + 'static
{
    let dialog = init_dialog(title);
    let child = build_child();
    let header = HeaderBar::builder()
        .css_classes(["header"])
        .show_end_title_buttons(true)
        .build();

    child.append(&header);
    child.append(&build_dialog_content(message, icon));

    let options = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .vexpand(false)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(20)
        .css_classes(["dialog-btns"])
        .build();

    let okbutton = Button::builder().css_classes(["dialog-ok-btn"]).label(ok_btn_label.unwrap_or("Proceed")).focusable(true).build();
    let cancelbutton = Button::builder().css_classes(["dialog-cancel-btn"]).label("Cancel").build();

    let wc = dialog.clone();
    let wc2 = dialog.clone();

    cancelbutton.connect_clicked(move |_| { wc.close(); });

    okbutton.connect_clicked(move |_| {
        wc2.close();
        f();
    });

    options.append(&add_clickable_animation_btn(cancelbutton));
    options.append(&add_clickable_animation_btn(okbutton.clone()));

    child.append(&options);
    dialog.set_child(Some(&child));

    dialog.set_focus(Some(&okbutton));

    dialog
}

pub fn init_regular_dialog(title: &str, message: &str, icon: &Image, ok_btn_label: Option<&str>) -> Dialog {
    let dialog = init_dialog(title);
    let child = build_child();
    let header = HeaderBar::builder()
        .css_classes(["header"])
        .show_end_title_buttons(true)
        .build();

    child.append(&header);
    child.append(&build_dialog_content(message, icon));

    let options = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .vexpand(false)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .spacing(20)
        .css_classes(["dialog-btns"])
        .build();

    let okbutton = Button::builder().css_classes(["dialog-ok-btn"]).label(ok_btn_label.unwrap_or("Ok")).focusable(true).build();

    let wc = dialog.clone();
    okbutton.connect_clicked(move |_| { wc.close(); });

    options.append(&add_clickable_animation_btn(okbutton.clone()));

    child.append(&options);
    dialog.set_child(Some(&child));
    dialog.set_focus(Some(&okbutton));

    dialog
}

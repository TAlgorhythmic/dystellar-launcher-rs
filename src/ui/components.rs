use gtk::{prelude::{GtkWindowExt, WidgetExt}, Button, Image};
use libadwaita::Bin;

use crate::{ui::dialog::{init_confirmation_dialog, init_regular_dialog}, utils::img::build_img_from_static_bytes};

pub static ICON_INFO: &'static [u8] = include_bytes!("./../../assets/icons/info.symbolic.png");
pub static ICON_ERROR: &'static [u8] = include_bytes!("./../../assets/icons/error.symbolic.png");

pub fn show_confirmation_dialog<F>(title: &str, msg: &str, oklabel: Option<&str>, icon: &'static [u8], run: F)
where 
    F: Fn() -> () + 'static
{
    let icon = build_img_from_static_bytes(icon).unwrap_or(Image::new());
    let dialog = init_confirmation_dialog(title, msg, &icon, run, oklabel);

    dialog.present();
}

pub fn show_dialog(title: &str, msg: &str, oklabel: Option<&str>, icon: &'static [u8]) {
    let icon = build_img_from_static_bytes(icon).unwrap_or(Image::new());
    let dialog = init_regular_dialog(title, msg, &icon, oklabel);

    dialog.present();
}

pub fn bin_wrap_btn(btn: Button) -> Bin {
    Bin::builder()
        .child(&btn)
        .hexpand(btn.hexpands())
        .vexpand(btn.vexpands())
        .css_classes(["bin-wrapper"])
        .halign(btn.halign())
        .valign(btn.valign())
        .overflow(gtk::Overflow::Visible)
        .build()
}

pub fn open_link_browser(url: &str) {
    let outlive_url: Box<str> = url.into();

    show_confirmation_dialog("Confirm Redirection", "This action will open your browser. Proceed?", None, ICON_INFO, move || {
        if let Err(err) = webbrowser::open(&outlive_url) {
            show_dialog("Default browser not found", format!("Failed to fetch default browser: {err}. Please open {outlive_url} manually on the web.").as_str(), None, ICON_ERROR);
        }
    });
}

use gtk::{prelude::{GtkWindowExt, WidgetExt}, Button, Image};
use libadwaita::Bin;

use crate::{ui::dialog::init_confirmation_dialog, utils::img::build_img_from_static_bytes};

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

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
        .width_request(btn.width())
        .height_request(btn.height())
        .hexpand(false)
        .vexpand(false)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .overflow(gtk::Overflow::Hidden)
        .build()
}

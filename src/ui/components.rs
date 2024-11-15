use std::sync::LazyLock;
use gtk::Widget;
use gtk::prelude::*;
use crate::ui::{MainUI, init_main_ui};

pub static mut MAIN_UI: LazyLock<MainUI> = LazyLock::new(|| init_main_ui());

pub fn info_btn_onpress(comp: &Widget) {
    comp.add_css_class("active");
}

pub fn info_btn_onrelease(comp: &Widget) {
    comp.remove_css_class("active");
}

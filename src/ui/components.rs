use std::sync::{Mutex, LazyLock};
use gtk::{Widget, EventControllerMotion};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::glib::*;
use crate::ui::{MainUI, init_main_ui};

pub static mut MAIN_UI: LazyLock<MainUI> = LazyLock::new(|| init_main_ui());
pub static WIDGET_STATES: LazyLock<Mutex<HashMap<String, bool>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn info_btn_onpress(comp: &Widget) {
    comp.add_css_class("info-btn-clicked");
}

pub fn info_btn_onrelease(comp: &Widget) {
    comp.remove_css_class("info-btn-clicked");
}

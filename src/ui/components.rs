use std::sync::{Mutex, LazyLock};
use gtk::{Widget, EventControllerMotion};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::glib::*;
use crate::ui::{UserInterfaceNormal, init_ui_normal};

pub static mut MAIN_UI: LazyLock<UserInterfaceNormal> = LazyLock::new(|| init_ui_normal());
pub static WIDGET_STATES: LazyLock<Mutex<HashMap<String, bool>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn hover_grow(comp: &Widget) {
    let name = comp.widget_name().to_string();
    WIDGET_STATES.lock().unwrap().insert(name, true);

    let width_initial = comp.width();
    let mut width = width_initial;
    let height_initial = comp.height();
    let mut height = height_initial;
    let target_size = 115;

    timeout_add_local(std::time::Duration::from_millis(25), || {
        if (width == target_size && height == target_size) || !WIDGET_STATES.lock().unwrap().get(&name).unwrap() {
            return ControlFlow::Break;
        } else {

            return ControlFlow::Continue;
        }
    });
}

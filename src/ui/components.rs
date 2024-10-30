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
    WIDGET_STATES.lock().unwrap().insert(name.clone(), true);

    let width_initial = comp.width();
    let height_initial = comp.height();
    let target_size = 115;
    let widget = comp.clone();

    timeout_add_local(std::time::Duration::from_millis(25), move || {
        let states = WIDGET_STATES.lock().unwrap();
        let width = widget.width();
        let height = widget.height();
        if (width == target_size && height == target_size) || !states.get(&name).unwrap() {
            return ControlFlow::Break;
        } else {
            let amount: i32 = ((target_size - width) as f32 * 0.23) as i32;
            widget.set_width_request(width + amount);
            widget.set_height_request(height + amount);
            return ControlFlow::Continue;
        }
    });
}

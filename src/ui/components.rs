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

pub fn hover_grow(comp: &Widget, target_size: i32, cut_margins: bool) {
    let name = comp.widget_name().to_string();
    WIDGET_STATES.lock().unwrap().insert(name.clone(), true);

    let widget = comp.clone();

    timeout_add_local(std::time::Duration::from_millis(25), move || {
        let states = WIDGET_STATES.lock().unwrap();
        let width = widget.width();
        let height = widget.height();

        if (width == target_size && height == target_size) || !states.get(&name).unwrap() {
            return ControlFlow::Break;
        } else {
            let mid_diff = target_size - width;
            let amount = (mid_diff as f32 * 0.4) as i32;
            let final_amount = if amount < 1 {1} else {amount};

            widget.set_width_request(width + final_amount);
            widget.set_height_request(height + final_amount);

            if cut_margins {
                widget.set_margin_end(widget.margin_end() - final_amount);
                widget.set_margin_start(widget.margin_start() - final_amount);
                widget.set_margin_top(widget.margin_top() - final_amount);
                widget.set_margin_bottom(widget.margin_bottom() - final_amount);
            }
            return ControlFlow::Continue;
        }
    });
}

pub fn hover_shrink(comp: &Widget, target_size: i32, add_margins: bool) {
    let name = comp.widget_name().to_string();
    WIDGET_STATES.lock().unwrap().insert(name.clone(), true);
    
    let widget = comp.clone();

    timeout_add_local(std::time::Duration::from_millis(25), move || {
        let states = WIDGET_STATES.lock().unwrap();
        let width = widget.width();
        let height = widget.height();

        if (width == target_size && height == target_size) || !states.get(&name).unwrap() {
            return ControlFlow::Break;
        } else {
            let diff = width - target_size;
            let amount = (diff as f32 * 0.4) as i32;
            let final_amount = if amount < 1 {1} else {amount};

            widget.set_width_request(width - final_amount);
            widget.set_height_request(height - final_amount);

            if add_margins {
                widget.set_margin_end(widget.margin_end() - final_amount);
                widget.set_margin_start(widget.margin_start() - final_amount);
                widget.set_margin_top(widget.margin_top() - final_amount);
                widget.set_margin_bottom(widget.margin_bottom() - final_amount);
            }
            return ControlFlow::Continue;
        }
    });
}

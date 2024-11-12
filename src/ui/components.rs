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

pub fn btn_onclick(comp: &Widget, target_width: i32, target_height: i32, restore_margins: bool) {

}

pub fn hover_grow(comp: &Widget, target_width: i32, target_height: i32, cut_margins: bool) {
    let name = comp.widget_name().to_string();
    WIDGET_STATES.lock().unwrap().insert(name.clone(), true);

    let widget = comp.clone();
    timeout_add_local(std::time::Duration::from_millis(25), move || {
        let states = WIDGET_STATES.lock().unwrap();
        let width = widget.width();
        let height = widget.height();

        if (width >= target_width && height >= target_height) || !states.get(&name).unwrap() {
            return ControlFlow::Break;
        } else {
            if width < target_width {
                let wid_diff = target_width - width;
                let amount_wid = (wid_diff as f32 * 0.4) as i32;
                let final_wid = if amount_wid < 1 {1} else {amount_wid};

                widget.set_width_request(width + final_wid);
                /*if cut_margins {
                    widget.set_margin_end(widget.margin_end() - final_wid);
                    widget.set_margin_start(widget.margin_start() - final_wid);
                }*/
            }
            if height < target_height {
                let he_diff = target_height - height;
                let amount_he = (he_diff as f32 * 0.4) as i32;
                let final_he = if amount_he < 1 {1} else {amount_he};

                widget.set_height_request(height + final_he);

                /*if cut_margins {
                    widget.set_margin_top(widget.margin_top() - final_he);
                    widget.set_margin_bottom(widget.margin_bottom() - final_he);
                }*/
            }
            return ControlFlow::Continue;
        }
    });
}

pub fn hover_shrink(comp: &Widget, target_width: i32, target_height: i32, add_margins: bool) {
    let name = comp.widget_name().to_string();
    WIDGET_STATES.lock().unwrap().insert(name.clone(), true);
    
    let widget = comp.clone();

    timeout_add_local(std::time::Duration::from_millis(25), move || {
        let states = WIDGET_STATES.lock().unwrap();
        let width = widget.width();
        let height = widget.height();

        if (width <= target_width && height <= target_height) || !states.get(&name).unwrap() {
            return ControlFlow::Break;
        } else {
            if width > target_width {
                let wid_diff = width - target_width;
                let amount_wid = (wid_diff as f32 * 0.4) as i32;
                let final_wid = if amount_wid < 1 {1} else {amount_wid};

                widget.set_width_request(width - final_wid);
                /*if add_margins {
                    widget.set_margin_end(widget.margin_end() - final_wid);
                    widget.set_margin_start(widget.margin_start() - final_wid);
                }*/
            }
            if height > target_height {
                let he_diff = height - target_height;
                let amount_he = (he_diff as f32 * 0.4) as i32;
                let final_he = if amount_he < 1 {1} else {amount_he};

                widget.set_height_request(height - final_he);
                /*if add_margins {
                    widget.set_margin_top(widget.margin_top() - final_he);
                    widget.set_margin_bottom(widget.margin_bottom() - final_he);
                }*/
            }
            return ControlFlow::Continue;
        }
    });
}

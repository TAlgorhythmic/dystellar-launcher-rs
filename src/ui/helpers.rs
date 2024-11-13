use gtk::prelude::*;
use gtk::{Button, EventControllerMotion, GestureClick};
use crate::ui::components::{info_btn_onpress, info_btn_onrelease};

pub fn add_link_controller_button(btn: &Button) {
    let link = EventControllerMotion::new();
    link.connect_enter(|event, _, _| {
        event.widget().map(|widget| {widget.set_cursor_from_name(Some("pointer"));});
    });
    link.connect_leave(|event| {
        event.widget().map(|widget| {widget.set_cursor_from_name(Some("none"));});
    });
    btn.add_controller(link);
}

pub fn add_info_btn_click_controller(btn: &Button) {
    let click = GestureClick::new();
    click.connect_begin(|event, _| {
        event.widget().map(|widget| info_btn_onpress(&widget));
    });
    click.connect_end(|event, _| {
        event.widget().map(|widget| info_btn_onrelease(&widget));
        event.set_state(gtk::EventSequenceState::None);
    });
    btn.add_controller(click);
}

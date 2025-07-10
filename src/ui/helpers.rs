use gtk::prelude::*;
use gtk::{Button, EventControllerMotion, GestureClick};

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

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

pub fn add_btn_click_controller(btn: &Button) -> GestureClick {
    let click = GestureClick::new();
    let ges = click.clone();

    click.connect_pressed(|event, _, _, _| {
        event.set_state(gtk::EventSequenceState::Claimed);
    });
    click.connect_released(|event, _, _, _| {
        event.set_state(gtk::EventSequenceState::None);
    });
    btn.add_controller(click);
    ges
}

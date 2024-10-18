use gtk::{ApplicationWindow, Button, Label, Box, Image, MenuButton, Popover, Grid};
use gtk::prelude::*;
use crate::ui;

// const NORMAL_UI: 

pub fn init_components(window: &ApplicationWindow) {
    // TODO: do better
    window.set_child(Some(&NORMAL_UI));
}

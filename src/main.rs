#![windows_subsystem = "windows"]

mod ui;
mod api;
mod logic;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/main_ui.rs"));
    include!(concat!(env!("OUT_DIR"), "/welcome_ui.rs"));
}

fn main() {
    ui::launcher::run();
}

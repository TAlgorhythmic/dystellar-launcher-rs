#![windows_subsystem = "windows"]

mod ui;
mod api;
mod utils;

fn main() {
    ui::launcher::run();
}

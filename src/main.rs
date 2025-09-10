#![windows_subsystem = "windows"]

mod ui;
mod api;
mod utils;
mod css;

fn main() {
    ui::launcher::run();
}

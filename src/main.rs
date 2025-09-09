#![windows_subsystem = "windows"]

mod ui;
mod api;
mod utils;
mod css;


slint::include_modules!();

fn main() {
    ui::launcher::run();
}

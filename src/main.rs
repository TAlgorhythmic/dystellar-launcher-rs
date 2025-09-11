#![windows_subsystem = "windows"]

mod ui;
mod api;
mod logic;

fn main() {
    ui::launcher::run();
}

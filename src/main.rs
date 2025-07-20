//#![windows_subsystem = "windows"]

use gtk::glib;
mod ui;
mod api;
mod utils;
mod css;

fn main() -> glib::ExitCode {
    ui::launcher::run()
}
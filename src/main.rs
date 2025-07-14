use gtk::glib;
mod ui;
mod css;
mod api;
mod utils;

fn main() -> glib::ExitCode {
    ui::launcher::run()
}

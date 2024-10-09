use gtk::glib;
mod ui;
mod css;
mod api;

fn main() -> glib::ExitCode {
    ui::launcher::run()
}

use gtk::glib;
mod ui;

fn main() -> glib::ExitCode {
    ui::launcher::run()
}

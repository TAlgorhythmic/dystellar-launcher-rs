use gtk::glib;
mod ui;
mod css;
mod api;
mod utils;

fn main() -> glib::ExitCode {
    if let Ok(gsk) = std::env::var("GSK_RENDERER") {
        println!("{gsk}");
    } else {
        println!("No backend specified.");
    }
    ui::launcher::run()
}

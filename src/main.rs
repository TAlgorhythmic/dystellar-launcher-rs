#![windows_subsystem = "windows"]

use std::error::Error;

mod ui;
mod api;
mod logic;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/main_ui.rs"));
    include!(concat!(env!("OUT_DIR"), "/welcome_ui.rs"));
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ui::launcher::run()?;
}

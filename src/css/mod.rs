use gtk::CssProvider;
use gtk::gdk::Display;
use crate::css;

pub mod buttons;
pub mod global;
pub mod components;

pub fn inject_css() {
    let provider = CssProvider::new();
    let complete_css: String = String::from(css::global::CSS) + css::buttons::CSS + css::components::CSS;

    provider.load_from_data(&complete_css.as_str());
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER
    );

}

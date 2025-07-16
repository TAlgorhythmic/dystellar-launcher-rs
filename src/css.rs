use gtk::CssProvider;
use gtk::gdk::Display;

static GLOBAL_CSS: &'static str = include_str!("../css_documents/global.css");
static BUTTONS_CSS: &'static str = include_str!("../css_documents/buttons.css");
static COMPONENTS_CSS: &'static str = include_str!("../css_documents/components.css");

pub fn inject_css() {
    let provider = CssProvider::new();
    let complete_css: String = String::from(GLOBAL_CSS) + BUTTONS_CSS + COMPONENTS_CSS;

    provider.load_from_data(&complete_css.as_str());
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER
    );

}

use std::fs;

fn main() {
    let global = fs::read_to_string("css_documents/global.css").expect("Error trying to read global.css.");
    let buttons = fs::read_to_string("css_documents/buttons.css").expect("Error trying to read buttons.css.");
    let components = fs::read_to_string("css_documents/components.css").expect("Error trying to read components.css.");

    let global_write = format!("pub const CSS: &str = \"{}\";", global);
    let buttons_write = format!("pub const CSS: &str = \"{}\";", buttons);
    let components_write = format!("pub const CSS: &str = \"{}\";", components);

    fs::write("src/css/global.rs", global_write).expect("Error trying to inject css.");
    fs::write("src/css/buttons.rs", buttons_write).expect("Error trying to inject css.");
    fs::write("src/css/components.rs", components_write).expect("Error trying to inject css.");
}

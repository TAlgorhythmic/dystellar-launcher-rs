use std::cell::RefCell;

use gtk::{prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt}, Box, Button, Label};
use libadwaita::{prelude::AdwWindowExt, Application, HeaderBar, Window};

use crate::{api::{control::http::login}, ui::{animations::add_clickable_animation_btn, components::build_inverted_dystellar_logo_img, launcher::{present_main_ui, SESSION}}, utils::img::build_img_from_static_bytes};

static MS_LOGO: &'static [u8] = include_bytes!("../../assets/icons/microsoft.png");

thread_local! {
    pub static IS_WAITING: RefCell<bool> = RefCell::new(false);
}

fn build_login_btn() -> Button {
    let content = Box::builder().orientation(gtk::Orientation::Horizontal).valign(gtk::Align::Fill).halign(gtk::Align::Fill).build();
    let icon = build_img_from_static_bytes(MS_LOGO).expect("Failed to load microsoft logo");
    let label = Label::builder().label("Microsoft").valign(gtk::Align::Center).halign(gtk::Align::Center).wrap(false).justify(gtk::Justification::Left).build();

    icon.set_halign(gtk::Align::Center);
    icon.set_valign(gtk::Align::Center);
    content.append(&icon);
    content.append(&label);

    Button::builder()
        .name("signin-microsoft-btn")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .focusable(false)
        .child(&content)
        .build()
}

fn build_welcome_content(window: Window, app: Application) -> Box {
    let content = Box::builder()
        .css_classes(["main-content"])
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .orientation(gtk::Orientation::Vertical)
        .build();

    let title = Label::builder().halign(gtk::Align::Center).valign(gtk::Align::Start).name("welcome-label").label("Welcome!").wrap(false).justify(gtk::Justification::Center).build();

    let inner_box = Box::builder().vexpand(true).valign(gtk::Align::Center).halign(gtk::Align::Center).orientation(gtk::Orientation::Vertical).spacing(0).build();
    let signin_label = Label::builder().label("Sign in with Microsoft:").name("signin-label").justify(gtk::Justification::Center).wrap(false).halign(gtk::Align::Center).build();
    let signin_btn = build_login_btn();
    let dyst_logo = build_inverted_dystellar_logo_img();
    signin_btn.connect_clicked(move |btn| {
        let is_waiting = IS_WAITING.with(|state| state.take());
        if is_waiting {
            return;
        }

        println!("{is_waiting}");

        let window_cl = window.clone();
        let app_cl = app.clone();

        btn.add_css_class("disabled");
        IS_WAITING.with(|state| state.replace(true));
        login(move |session| {
            SESSION.set(Some(session));
            window_cl.close();
            present_main_ui(&app_cl);
        });
    });
    dyst_logo.set_valign(gtk::Align::End);
    dyst_logo.set_halign(gtk::Align::Center);
    dyst_logo.set_widget_name("welcome-logo");

    inner_box.append(&signin_label);
    inner_box.append(&add_clickable_animation_btn(signin_btn));

    content.append(&title);
    content.append(&inner_box);
    content.append(&dyst_logo);

    content
}

pub fn welcome_login_screen(app: &Application) -> Window {
    let window = Window::builder()
        .name("WelcomeScreen")
        .title("Welcome to Dystellar Launcher")
        .default_width(700)
        .default_height(760)
        .css_classes(["window"])
        .decorated(false)
        .resizable(false)
        .application(app)
        .build();

    let parent = Box::builder().halign(gtk::Align::Fill).valign(gtk::Align::Fill).orientation(gtk::Orientation::Vertical).build();
    let header = HeaderBar::builder()
        .css_classes(["header"])
        .build();

    parent.append(&header);
    parent.append(&build_welcome_content(window.clone(), app.clone()));

    window.set_content(Some(&parent));

    window
}

pub mod launcher;
pub mod components;
pub mod helpers;
use gtk::prelude::*;
use gtk::gdk_pixbuf::Pixbuf;

pub struct MainUI {
    main_content: gtk::Box,
    subheader: gtk::CenterBox,
    info_holder: gtk::Box,
    settings_btn: gtk::Button,
    store_btn: gtk::Button,
    tos_btn: gtk::Button,
    acc_holder: gtk::Box,
    acc_btn: gtk::Button,
    acc_popover: gtk::PopoverMenu,
    login_btn: gtk::Button,
    logout_btn: gtk::Button,
    switch_acc_btn: gtk::Button,
    help_btn: gtk::Button,
    central_content: gtk::Grid,
    updates_grid: gtk::Grid,
    updates_next_btn: gtk::Button,
    updates_previous_btn: gtk::Button,
    updates_main_btn: gtk::Button,
    gamestate_box: gtk::Box,
    center_img: gtk::Image,
    launch_btn: gtk::Button,
    mods_btn: gtk::Button,
    events_grid: gtk::Grid,
    events_next_btn: gtk::Button,
    events_previous_btn: gtk::Button,
    events_main_btn: gtk::Button,
    footer: gtk::CenterBox,
    socials_box: gtk::Box,
    d_btn: gtk::Button,
    y_btn: gtk::Button,
    x_btn: gtk::Button,
    dyst_box: gtk::Box,
    dyst_logo: gtk::Image,
    dyst_label: gtk::Label
}

impl MainUI {}

pub fn init_main_ui() -> MainUI {
    let main_content: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).css_classes(["main-content"]).vexpand(true).hexpand(true).valign(gtk::Align::Fill).halign(gtk::Align::Fill).build();
    let subheader: gtk::CenterBox = gtk::CenterBox::builder().orientation(gtk::Orientation::Horizontal).css_classes(["subheader"]).build();
    let info_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let settings_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["info-btn"]).build(); //TODO: icon
    let store_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["info-btn"]).build(); //TODO: icon
    let tos_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["info-btn"]).build(); //TODO: icon
    let acc_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let acc_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["info-btn"]).build(); //TODO: icon
    let acc_popover: gtk::PopoverMenu = gtk::PopoverMenu::builder().focusable(false).css_classes(["popover"]).build();
    let login_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["popover-btn"]).build();
    let logout_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["popover-btn"]).build();
    let switch_acc_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["popover-btn"]).build();
    let help_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["popover-btn"]).build();
    let central_content: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).row_homogeneous(true).column_homogeneous(true).hexpand(true).vexpand(true).css_classes(["central-content"]).build();
    let updates_grid: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).css_classes(["content-grid"]).hexpand(true).vexpand(true).build();
    let updates_next_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).css_classes(["next-btn"]).build();
    let updates_previous_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).css_classes(["previous-btn"]).build();
    let updates_main_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).hexpand(true).css_classes(["web-content"]).build();
    let gamestate_box: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(4).valign(gtk::Align::Center).css_classes(["central-box"]).build();
    let center_img: gtk::Image = gtk::Image::new();
    let launch_btn: gtk::Button = gtk::Button::builder().focusable(false).label("Loading...").css_classes(["launch-btn", "disabled"]).build();
    let mods_btn: gtk::Button = gtk::Button::builder().focusable(false).label("Mods").css_classes(["mods-btn"]).build();
    let events_grid: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).hexpand(true).vexpand(true).css_classes(["content-grid"]).build();
    let events_next_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).css_classes(["next-btn"]).build();
    let events_previous_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).css_classes(["previous-btn"]).build();
    let events_main_btn: gtk::Button = gtk::Button::builder().focusable(false).vexpand(true).hexpand(true).css_classes(["web-content"]).build();
    let footer: gtk::CenterBox = gtk::CenterBox::builder().orientation(gtk::Orientation::Horizontal).css_classes(["footer"]).build();
    let socials_box: gtk::Box = gtk::Box::builder().css_classes(["info-holder"]).hexpand(false).orientation(gtk::Orientation::Horizontal).build();
    let d_btn: gtk::Button = gtk::Button::builder().margin_start(18).margin_end(4).focusable(false).css_classes(["info-btn"]).build();
    let y_btn: gtk::Button = gtk::Button::builder().focusable(false).css_classes(["info-btn"]).build();
    let x_btn: gtk::Button = gtk::Button::builder().margin_end(18).margin_start(4).focusable(false).css_classes(["info-btn"]).build();
    let dyst_box: gtk::Box = gtk::Box::builder().focusable(false).orientation(gtk::Orientation::Horizontal).css_classes(["info-holder"]).build();
    let dyst_logo: gtk::Image = gtk::Image::builder().build();
    let dyst_label: gtk::Label = gtk::Label::builder().css_classes(["label"]).label("Dystellar Network").build();
    
    // Set hierarchy
    info_holder.append(&settings_btn);
    info_holder.append(&store_btn);
    info_holder.append(&tos_btn);
    acc_popover.add_child(&login_btn, "Log In");
    acc_popover.add_child(&help_btn, "Help");
    acc_btn.set_child(Some(&acc_popover));
    acc_holder.append(&acc_btn);
    subheader.set_start_widget(Some(&info_holder));
    subheader.set_end_widget(Some(&acc_holder));

    gamestate_box.append(&launch_btn);
    gamestate_box.append(&mods_btn);

    updates_grid.attach(&updates_main_btn, 0, 0, 6, 1);
    updates_grid.attach(&updates_previous_btn, 0, 0, 1, 1);
    updates_grid.attach(&updates_next_btn, 5, 0, 1, 1);
    
    events_grid.attach(&events_main_btn, 0, 0, 6, 1);
    events_grid.attach(&events_previous_btn, 0, 0, 1, 1);
    events_grid.attach(&events_next_btn, 5, 0, 1, 1);
    
    central_content.attach(&updates_grid, 0, 0, 1, 1);
    central_content.attach(&gamestate_box, 1, 0, 1, 1);
    central_content.attach(&events_grid, 2, 0, 1, 1);
    
    socials_box.append(&d_btn);
    socials_box.append(&y_btn);
    socials_box.append(&x_btn);

    dyst_box.append(&dyst_logo);
    dyst_box.append(&dyst_label);

    footer.set_start_widget(Some(&socials_box));
    footer.set_end_widget(Some(&dyst_box));
    
    main_content.append(&subheader);
    main_content.append(&central_content);
    main_content.append(&footer);
    
    let ui = MainUI {
        main_content, subheader, info_holder, settings_btn, store_btn, tos_btn, acc_holder,
        acc_btn, acc_popover, login_btn, logout_btn, switch_acc_btn, help_btn, central_content,
        updates_grid, updates_next_btn, updates_previous_btn, updates_main_btn, gamestate_box,
        center_img, launch_btn, mods_btn, events_grid, events_next_btn, events_previous_btn,
        events_main_btn, footer, socials_box, d_btn, y_btn, x_btn, dyst_box, dyst_logo,dyst_label
    };
    init_icons(&ui);
    add_events(&ui);
    ui
}

fn init_icons(ui: &MainUI) {
    let f_x;
    let f_d;
    let f_y;
    let settings;
    let previous;
    let next;

    // Windows, why?
    #[cfg(target_os = "windows")] {
        f_x = include_bytes!(".\\..\\..\\assets\\icons\\x.symbolic.png");
        f_d = include_bytes!(".\\..\\..\\assets\\icons\\discord.symbolic.png");
        f_y = include_bytes!(".\\..\\..\\assets\\icons\\youtube.symbolic.png");
        settings = include_bytes!(".\\..\\..\\assets\\icons\\settings.symbolic.png");
        previous = include_bytes!(".\\..\\..\\assets\\icons\\previous.symbolic.png");
        next = include_bytes!(".\\..\\..\\assets\\icons\\next.symbolic.png");
    }
    #[cfg(not(target_os = "windows"))] {
        f_x = include_bytes!("./../../assets/icons/x.symbolic.png");
        f_d = include_bytes!("./../../assets/icons/discord.symbolic.png");
        f_y = include_bytes!("./../../assets/icons/youtube.symbolic.png");
        settings = include_bytes!("./../../assets/icons/settings.symbolic.png");
        previous = include_bytes!("./../../assets/icons/previous.symbolic.png");
        next = include_bytes!("./../../assets/icons/next.symbolic.png");
    }

    let bytes_x = gtk::glib::Bytes::from_owned(f_x);
    let bytes_d = gtk::glib::Bytes::from_owned(f_d);
    let bytes_y = gtk::glib::Bytes::from_owned(f_y);
    let bytes_sett = gtk::glib::Bytes::from_owned(settings);
    let bytes_prev = gtk::glib::Bytes::from_owned(previous);
    let bytes_next = gtk::glib::Bytes::from_owned(next);

    let stream_x = gtk::gio::MemoryInputStream::from_bytes(&bytes_x);
    let stream_d = gtk::gio::MemoryInputStream::from_bytes(&bytes_d);
    let stream_y = gtk::gio::MemoryInputStream::from_bytes(&bytes_y);
    let stream_sett = gtk::gio::MemoryInputStream::from_bytes(&bytes_sett);
    let stream_prev = gtk::gio::MemoryInputStream::from_bytes(&bytes_prev);
    let stream_next = gtk::gio::MemoryInputStream::from_bytes(&bytes_next);

    let pixbuf_x = Pixbuf::from_stream(&stream_x, gtk::gio::Cancellable::NONE);
    let pixbuf_d = Pixbuf::from_stream(&stream_d, gtk::gio::Cancellable::NONE);
    let pixbuf_y = Pixbuf::from_stream(&stream_y, gtk::gio::Cancellable::NONE);
    let pixbuf_sett = Pixbuf::from_stream(&stream_sett, gtk::gio::Cancellable::NONE);
    let pixbuf_prev = Pixbuf::from_stream(&stream_prev, gtk::gio::Cancellable::NONE).expect("Pixbuf error prev.");
    let pixbuf_next = Pixbuf::from_stream(&stream_next, gtk::gio::Cancellable::NONE).expect("Pixbuf error next.");

    let img_x = gtk::Image::new(); img_x.set_from_pixbuf(Some(&pixbuf_x.expect("Pixbuf error X.")));
    let img_d = gtk::Image::new(); img_d.set_from_pixbuf(Some(&pixbuf_d.expect("Pixbuf error D.")));
    let img_y = gtk::Image::new(); img_y.set_from_pixbuf(Some(&pixbuf_y.expect("Pixbuf error Y.")));
    let img_sett = gtk::Image::new(); img_sett.set_from_pixbuf(Some(&pixbuf_sett.expect("Pixbuf error sett.")));

    ui.y_btn.set_child(Some(&img_y));
    ui.d_btn.set_child(Some(&img_d));
    ui.x_btn.set_child(Some(&img_x));
    ui.settings_btn.set_child(Some(&img_sett));
    ui.updates_previous_btn.set_child(Some(&gtk::Image::from_pixbuf(Some(&pixbuf_prev))));
    ui.events_previous_btn.set_child(Some(&gtk::Image::from_pixbuf(Some(&pixbuf_prev))));
    ui.updates_next_btn.set_child(Some(&gtk::Image::from_pixbuf(Some(&pixbuf_next))));
    ui.events_next_btn.set_child(Some(&gtk::Image::from_pixbuf(Some(&pixbuf_next))));
}

fn add_events(ui: &MainUI) {
    helpers::add_link_controller_button(&ui.x_btn);
    helpers::add_link_controller_button(&ui.d_btn);
    helpers::add_link_controller_button(&ui.y_btn);
    helpers::add_link_controller_button(&ui.launch_btn);
    helpers::add_btn_click_controller(&ui.x_btn);
    helpers::add_btn_click_controller(&ui.d_btn);
    helpers::add_btn_click_controller(&ui.y_btn);
    helpers::add_btn_click_controller(&ui.settings_btn);
    helpers::add_btn_click_controller(&ui.updates_next_btn);
    helpers::add_btn_click_controller(&ui.events_next_btn);
    helpers::add_btn_click_controller(&ui.updates_previous_btn);
    helpers::add_btn_click_controller(&ui.events_previous_btn);
    helpers::add_btn_click_controller(&ui.launch_btn);
    helpers::add_btn_click_controller(&ui.mods_btn);
    helpers::add_btn_click_controller(&ui.tos_btn);
    
    let updates_previous_btn = ui.updates_previous_btn.clone();
    let updates_next_btn = ui.updates_next_btn.clone();

    let updates = gtk::EventControllerMotion::new();
    updates.connect_enter(move |_, _, _| {
        updates_previous_btn.add_css_class("focus");
        updates_next_btn.add_css_class("focus");
    });

    let updates_previous_btn = ui.updates_previous_btn.clone();
    let updates_next_btn = ui.updates_next_btn.clone();

    updates.connect_leave(move |_| {
        updates_previous_btn.remove_css_class("focus");
        updates_next_btn.remove_css_class("focus");
    });
    
    let events = gtk::EventControllerMotion::new();

    let events_previous_btn = ui.events_previous_btn.clone();
    let events_next_btn = ui.events_next_btn.clone();

    events.connect_enter(move |_, _, _| {
        events_previous_btn.add_css_class("focus");
        events_next_btn.add_css_class("focus");
    });

    let events_previous_btn = ui.events_previous_btn.clone();
    let events_next_btn = ui.events_next_btn.clone();

    events.connect_leave(move |_| {
        events_previous_btn.remove_css_class("focus");
        events_next_btn.remove_css_class("focus");
    });

    ui.updates_grid.add_controller(updates);
    ui.events_grid.add_controller(events);
}

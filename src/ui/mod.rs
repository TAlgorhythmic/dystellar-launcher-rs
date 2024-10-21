pub mod launcher;
pub mod components;
pub mod icons;
use gtk::prelude::*;

pub struct UserInterfaceNormal {
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

impl UserInterfaceNormal {}

pub fn init_ui_normal() -> UserInterfaceNormal {
    let main_content: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).margin_end(16).margin_top(16).margin_start(16).margin_bottom(16).css_classes(["main-content"]).vexpand(true).hexpand(true).valign(gtk::Align::Fill).halign(gtk::Align::Fill).build();
    let subheader: gtk::CenterBox = gtk::CenterBox::builder().orientation(gtk::Orientation::Horizontal).css_classes(["subheader"]).build();
    let info_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let settings_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let store_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let tos_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let acc_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let acc_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let acc_popover: gtk::PopoverMenu = gtk::PopoverMenu::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["popover"]).build();
    let login_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["popover-btn"]).build();
    let logout_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["popover-btn"]).build();
    let switch_acc_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["popover-btn"]).build();
    let help_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["popover-btn"]).build();
    let central_content: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).row_homogeneous(true).column_homogeneous(true).hexpand(true).vexpand(true).css_classes(["central-content"]).build();
    let updates_grid: gtk::Grid = gtk::Grid::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).orientation(gtk::Orientation::Horizontal).css_classes(["content-grid"]).hexpand(true).vexpand(true).build();
    let updates_next_btn: gtk::Button = gtk::Button::builder().vexpand(true).css_classes(["next-btn"]).build();
    let updates_previous_btn: gtk::Button = gtk::Button::builder().vexpand(true).css_classes(["previous-btn"]).build();
    let updates_main_btn: gtk::Button = gtk::Button::builder().vexpand(true).hexpand(true).css_classes(["web-content"]).build();
    let gamestate_box: gtk::Box = gtk::Box::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).orientation(gtk::Orientation::Vertical).spacing(4).valign(gtk::Align::Center).css_classes(["central-grid"]).build();
    let center_img: gtk::Image = gtk::Image::new();
    let launch_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).label("Loading...").css_classes(["launch-btn", "disabled"]).build();
    let mods_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).label("Mods").css_classes(["mods-btn"]).build();
    let events_grid: gtk::Grid = gtk::Grid::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).orientation(gtk::Orientation::Horizontal).hexpand(true).vexpand(true).css_classes(["content-grid"]).build();
    let events_next_btn: gtk::Button = gtk::Button::builder().vexpand(true).css_classes(["next-btn"]).build();
    let events_previous_btn: gtk::Button = gtk::Button::builder().vexpand(true).css_classes(["next-btn"]).build();
    let events_main_btn: gtk::Button = gtk::Button::builder().vexpand(true).hexpand(true).css_classes(["web-content"]).build();
    let footer: gtk::CenterBox = gtk::CenterBox::builder().orientation(gtk::Orientation::Horizontal).css_classes(["footer"]).build();
    let socials_box: gtk::Box = gtk::Box::builder().css_classes(["info-holder"]).orientation(gtk::Orientation::Horizontal).spacing(2).build();
    let d_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let y_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let x_btn: gtk::Button = gtk::Button::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["info-btn"]).build(); //TODO: icon
    let dyst_box: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let dyst_logo: gtk::Image = gtk::Image::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).build();
    let dyst_label: gtk::Label = gtk::Label::builder().margin_bottom(10).margin_top(10).margin_start(10).margin_end(10).css_classes(["label"]).label("Dystellar Network").build();
    
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

    updates_grid.attach(&updates_main_btn, 0, 0, 3, 1);
    updates_grid.attach(&updates_next_btn, 2, 0, 1, 1);
    updates_grid.attach(&updates_previous_btn, 0, 0, 1, 1);
    
    events_grid.attach(&events_main_btn, 0, 0, 3, 1);
    events_grid.attach(&events_previous_btn, 0, 0, 1, 1);
    events_grid.attach(&events_next_btn, 2, 0, 1, 1);
    
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
    
    return UserInterfaceNormal {
        main_content,
        subheader,
        info_holder,
        settings_btn,
        store_btn,
        tos_btn,
        acc_holder,
        acc_btn,
        acc_popover,
        login_btn,
        logout_btn,
        switch_acc_btn,
        help_btn,
        central_content,
        updates_grid,
        updates_next_btn,
        updates_previous_btn,
        updates_main_btn,
        gamestate_box,
        center_img,
        launch_btn,
        mods_btn,
        events_grid,
        events_next_btn,
        events_previous_btn,
        events_main_btn,
        footer,
        socials_box,
        d_btn,
        y_btn,
        x_btn,
        dyst_box,
        dyst_logo,
        dyst_label
    }
}

pub mod launcher;
pub mod components;
pub mod icons;
use gtk::prelude::*;

pub struct UserInterfaceNormal {
    main_content: gtk::Grid,
    header: gtk::Box,
    quit_btn: gtk::Button,
    maximize_btn: gtk::Button,
    minimize_btn: gtk::Button,
    subheader: gtk::Box,
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
    gamestate_grid: gtk::Grid,
    center_img: gtk::Image,
    launch_btn: gtk::Button,
    mods_btn: gtk::Button,
    events_grid: gtk::Grid,
    events_next_btn: gtk::Button,
    events_previous_btn: gtk::Button,
    events_main_btn: gtk::Button,
    footer: gtk::Box,
    socials_box: gtk::Box,
    d_btn: gtk::Button,
    y_btn: gtk::Button,
    x_btn: gtk::Button,
    dyst_logo: gtk::Image,
    dyst_label: gtk::Label
}

pub fn init_ui_normal() {
    let main_content: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Vertical).row_spacing(5).hexpand_set(true).css_classes(["main-content"]).build();
    let header: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["header"]).build();
    let quit_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let maximize_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let minimize_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let subheader: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["subheader"]).build();
    let info_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let settings_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let store_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let tos_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let acc_holder: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(2).css_classes(["info-holder"]).build();
    let acc_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let acc_popover: gtk::PopoverMenu = gtk::PopoverMenu::builder().css_classes(["popover"]).build();
    let login_btn: gtk::Button = gtk::Button::builder().css_classes(["popover-btn"]).build();
    let logout_btn: gtk::Button = gtk::Button::builder().css_classes(["popover-btn"]).build();
    let switch_acc_btn: gtk::Button = gtk::Button::builder().css_classes(["popover-btn"]).build();
    let help_btn: gtk::Button = gtk::Button::builder().css_classes(["popover-btn"]).build();
    let central_content: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).row_homogeneous(true).column_homogeneous(true).css_classes(["central-content"]).build();
    let updates_grid: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).css_classes(["content-grid"]).build();
    let updates_next_btn: gtk::Button = gtk::Button::builder().css_classes(["next-btn"]).build();
    let updates_previous_btn: gtk::Button = gtk::Button::builder().css_classes(["previous-btn"]).build();
    let updates_main_btn: gtk::Button = gtk::Button::builder().css_classes(["web-content"]).build();
    let gamestate_grid: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Vertical).css_classes(["central-grid"]).build();
    // let center_img: gtk::Image;
    let launch_btn: gtk::Button = gtk::Button::builder().label("Loading...").css_classes(["launch-btn", "disabled"]).build();
    let mods_btn: gtk::Button = gtk::Button::builder().label("Mods").css_classes(["mods-btn"]).build();
    let events_grid: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Horizontal).css_classes(["content-grid"]).build();
    let events_next_btn: gtk::Button = gtk::Button::builder().css_classes(["next-btn"]).build();
    let events_previous_btn: gtk::Button = gtk::Button::builder().css_classes(["next-btn"]).build();
    let events_main_btn: gtk::Button = gtk::Button::builder().css_classes(["web-content"]).build();
    let footer: gtk::Box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).css_classes(["footer"]).spacing(2).build();
    let socials_box: gtk::Box = gtk::Box::builder().css_classes(["info-holder"]).orientation(gtk::Orientation::Horizontal).spacing(2).build();
    let d_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let y_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let x_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    // let dyst_logo: gtk::Image;
    let dyst_label: gtk::Label = gtk::Label::builder().css_classes(["label"]).label("Dystellar Network").build();
    
    // Set hierarchy
    header.append(&quit_btn);
    header.append(&maximize_btn);
    header.append(&minimize_btn);
    info_holder.append(&settings_btn);
    info_holder.append(&store_btn);
    info_holder.append(&tos_btn);
    acc_popover.add_child(&login_btn, "Log In");
    acc_popover.add_child(&help_btn, "Help");
    acc_btn.set_child(Some(&acc_popover));
    acc_holder.append(&acc_btn);
    subheader.append(&info_holder);
    subheader.append(&acc_holder);

    gamestate_grid.attach();

    updates_grid.attach(&updates_main_btn, 0, 0, 3, 1);
    updates_grid.attach(&updates_next_btn, 2, 0, 1, 1);
    updates_grid.attach(&updates_previous_btn, 0, 0, 1, 1);
    
    events_grid.attach(&events_main_btn, 0, 0, 3, 0);
    events_grid.attach(&events_previous_btn, 0, 0, 1, 1);
    events_grid.attach(&events_next_btn, 2, 0, 1, 1);

    central_content.attach(&updates_grid, 0, 0, 1, 1);
    central_content.attach(&gamestate_grid, 1, 0, 1, 1);
    central_content.attach(&events_grid, 2, 0, 1, 1);
    main_content.attach(&header, 0, 0, 1, 1);
    main_content.attach(&subheader, 0, 1, 1, 1);
    main_content.attach(&central_content, 0, 2, 1, 1);
    main_content.attach(&footer, 0, 3, 1, 1);

}

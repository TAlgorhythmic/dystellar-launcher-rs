pub mod launcher;
pub mod components;
pub mod icons;

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

pub fn init_ui_normal() -> UserInterfaceNormal {
    let main_content: gtk::Grid = gtk::Grid::builder().orientation(gtk::Orientation::Vertical).css_name("main-content").build();
    let header: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let quit_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let maximize_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let minimize_btn: gtk::Button = gtk::Button::builder().css_classes(["header-btn"]).build(); //TODO: icon
    let subheader: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let info_holder: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 2);
    let settings_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let store_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon
    let tos_btn: gtk::Button = gtk::Button::builder().css_classes(["info-btn"]).build(); //TODO: icon

}

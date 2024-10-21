use std::sync::LazyLock;
use crate::ui::{UserInterfaceNormal, init_ui_normal};

pub static mut MAIN_UI: LazyLock<UserInterfaceNormal> = LazyLock::new(|| init_ui_normal());

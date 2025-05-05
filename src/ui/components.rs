use std::sync::LazyLock;
use crate::ui::{MainUI, init_main_ui};

pub const MAIN_UI: LazyLock<MainUI> = LazyLock::new(|| init_main_ui());

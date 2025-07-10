use std::sync::LazyLock;

use crate::ui::main_ui::{init_main_ui, MainUI};

pub const MAIN_UI: LazyLock<MainUI> = LazyLock::new(|| init_main_ui());

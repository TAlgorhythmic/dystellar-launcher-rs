use std::{path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;

static PROJECT_DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| ProjectDirs::from("org.dystellar", "mmorpg", "Launcher").expect("Unable to provide respective OS directories")); 

pub fn get_cache_dir() -> PathBuf {
    PROJECT_DIRS.cache_dir().to_path_buf()
}

pub fn get_data_dir() -> PathBuf {
    PROJECT_DIRS.data_dir().to_path_buf()
}

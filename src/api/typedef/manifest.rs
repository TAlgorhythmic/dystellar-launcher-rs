use std::error::Error;

use json::JsonValue;

pub struct AssetIndex {
    id: Box<str>,
    sha1: Box<str>,
    size: isize,
    total_size: isize,
    url: Box<str>
}

pub struct Download {
    id: Box<str>,
    path: Option<Box<str>>,
    sha1: Box<str>,
    size: isize,
    url: Box<str>
}

pub struct Library {
    pub downloads: Vec<Download>,
    pub name: Box<str>,
    pub os: Vec<Box<str>>
}

pub struct MinecraftManifest {
    pub args: Vec<Box<str>>,
    pub compliance_level: i32,
    pub downloads: Vec<Download>,
    pub java_version: i32,
    pub libs: Vec<Library>,
    pub main_class: Box<str>
}

impl TryFrom<JsonValue> for MinecraftManifest {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        
    }
}

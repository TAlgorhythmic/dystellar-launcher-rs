use std::{error::Error, fs, str::FromStr};

use json::{JsonValue, object, stringify_pretty};

use crate::api::control::dir_provider::{get_cache_dir, get_data_dir};

pub struct Size { x: i32, y: i32 }

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err("expected 'XxY' format".into());
        }

        let x = parts[0].trim().parse::<i32>()
            .map_err(|_| "invalid x")?;
        let y = parts[1].trim().parse::<i32>()
            .map_err(|_| "invalid y")?;

        Ok(Size { x, y })
    }
}

pub struct Config {
    ram_allocation_mb: i32,
    resolution: Size,
    fullscreen: bool,
    branch: Box<str>,
    jvm_args: Box<str>,
    close_on_launch: bool,
    game_dir: Box<str>,
    cache_dir: Box<str>,
    jdk_dir: Box<str>
}

impl From<&Config> for JsonValue {
    fn from(v: &Config) -> Self {
        object! {
            ram_allocation_mb: v.ram_allocation_mb,
            resolution: format!("{}x{}", v.resolution.x, v.resolution.y),
            fullscreen: v.fullscreen,
            branch: v.branch.clone().into_string(),
            jvm_args: v.jvm_args.clone().into_string(),
            close_on_launch: v.close_on_launch,
            game_dir: v.game_dir.clone().into_string(),
            cache_dir: v.cache_dir.clone().into_string(),
            jdk_dir: v.jdk_dir.clone().into_string()
        }
    }
}

impl From<Config> for JsonValue {
    fn from(v: Config) -> Self {
        JsonValue::from(&v)
    }
}

impl TryFrom<JsonValue> for Config {
    type Error = json::Error;

    fn try_from(json: JsonValue) -> Result<Self, Self::Error> {
        Ok(Self {
            ram_allocation_mb: json["ram_allocation_mb"].as_i32().ok_or(json::Error::WrongType(String::from("Wrong json")))?,
            resolution: json["resolution"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.parse().map_err(|e: String| json::Error::WrongType(e))?,
            fullscreen: json["fullscreen"].as_bool().ok_or(json::Error::WrongType(String::from("Wrong json")))?,
            branch: json["branch"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.into(),
            jvm_args: json["jvm_args"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.into(),
            close_on_launch: json["close_on_launch"].as_bool().ok_or(json::Error::WrongType(String::from("Wrong json")))?,
            game_dir: json["game_dir"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.into(),
            cache_dir: json["cache_dir"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.into(),
            jdk_dir: json["jdk_dir"].as_str().ok_or(json::Error::WrongType(String::from("Wrong json")))?.into()
        })
    }
}

impl Config {
    pub fn default() -> Self {
        Self {
            ram_allocation_mb: 3072,
            resolution: Size { x: 854, y: 480 },
            fullscreen: false,
            branch: "master".into(),
            jvm_args: "".into(),
            close_on_launch: false,
            game_dir: format!("{}/.dystellar", get_data_dir().to_str().unwrap()).into(),
            cache_dir: format!("{}/dyst", get_cache_dir().to_str().unwrap()).into(),
            jdk_dir: format!("{}/.dystellar/jdk", get_data_dir().to_str().unwrap()).into()
        }
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        todo!()
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        fs::write(path, stringify_pretty(self, 2))?;

        Ok(())
    }
}

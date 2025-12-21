use std::{cmp::Ordering, error::Error};

use json::JsonValue;

pub struct AssetIndex {
    id: Box<str>,
    sha1: Box<str>,
    size: isize,
    total_size: isize,
    url: Box<str>
}

pub struct Download {
    pub id: Box<str>,
    pub path: Option<Box<str>>,
    pub sha1: Box<str>,
    pub size: isize,
    pub url: Box<str>
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

pub struct JavaManifest {
    pub download_url: Box<str>,
    pub name: Box<str>
}

impl TryFrom<JsonValue> for JavaManifest {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
        let max = v.members().max_by(|v1, v2| {
            let name1 = v1["name"].as_str();
            let name2 = v2["name"].as_str();

            if name1.is_none() {
                return Ordering::Less;
            } else if name2.is_none() {
                return Ordering::Greater;
            }

            let name1 = name1.unwrap();
            let name2 = name2.unwrap();

            name1.cmp(name2)
        }).ok_or_else(|| -> Self::Error {"Failed to find a suitable jre".into()})?;

        Ok(Self {
            download_url: max["download_url"].as_str().ok_or_else(|| -> Self::Error {"Failed to find a suitable jre".into()})?.into(),
            name: max["name"].as_str().ok_or_else(|| -> Self::Error {"Failed to find a suitable jre".into()})?.into(),
        })
    }
}

impl TryFrom<JsonValue> for AssetIndex {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
        Ok(Self {
            id: v["id"].as_str().ok_or("assetIndex.id missing")?.into(),
            sha1: v["sha1"].as_str().ok_or("assetIndex.sha1 missing")?.into(),
            size: v["size"].as_isize().ok_or("assetIndex.size missing")?,
            total_size: v["totalSize"].as_isize().ok_or("assetIndex.totalSize missing")?,
            url: v["url"].as_str().ok_or("assetIndex.url missing")?.into(),
        })
    }
}

impl TryFrom<(&str, &JsonValue)> for Download {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(v: (&str, &JsonValue)) -> Result<Self, Self::Error> {
        let (id, v) = v;
        Ok(Self {
            id: id.into(),
            path: v["path"].as_str().map(|s| s.into()),
            sha1: v["sha1"].as_str().ok_or("download.sha1 missing")?.into(),
            size: v["size"].as_isize().ok_or("download.size missing")?,
            url: v["url"].as_str().ok_or("download.url missing")?.into(),
        })
    }
}

impl TryFrom<JsonValue> for Library {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
        let mut dl_vec = vec![];

        for item in v["downloads"].entries() {
            dl_vec.push(Download::try_from(item)?);
        }

        // OS rules
        let mut os_vec = vec![];

        if let JsonValue::Array(rules) = &v["rules"] {
            for rule in rules {
                if rule["action"] == "allow" {
                    if let Some(name) = rule["os"]["name"].as_str() {
                        os_vec.push(name.into());
                    }
                }
            }
        }

        Ok(Self {
            downloads: dl_vec,
            name: v["name"].as_str().ok_or("library.name missing")?.into(),
            os: os_vec,
        })
    }
}

impl TryFrom<JsonValue> for MinecraftManifest {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let mut downloads: Vec<Download> = vec![];
        let mut libs: Vec<Library> = vec![];

        for download in value["downloads"].entries() {
            downloads.push(Download::try_from(download)?);
        }
        for lib in value["libraries"].members() {
            libs.push(Library::try_from(lib.clone())?);
        }

        Ok(Self {
            args: vec![],
            compliance_level: value["complianceLevel"].as_i32().ok_or("complianceLevel missing")?,
            downloads,
            java_version: value["javaVersion"]["majorVersion"].as_i32().ok_or("javaVersion.majorVersion missing")?,
            libs,
            main_class: value["mainClass"].as_str().ok_or("mainClass missing")?.into()
        })
    }
}

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DownloadInfo {
    name: String,
    icon: Option<String>,
    url: Option<String>,
    filename: Option<String>,
    gold_color: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlatformInfo {
    title: String,
    platform_badge: Option<String>,
    platform_key: String,
    downloads: Vec<DownloadInfo>,
}

// This contains a bunch of stuff that various pages want to reference.
// Little point in restricting certain data to certain pages since we're a static generator
// so it'll be a grab bag of stuff.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct GlobalMeta {
    pub app_version: String,
}

impl GlobalMeta {
    pub fn new() -> anyhow::Result<Self> {
        // Parse the download path dump.

        let downloads_json = std::fs::read_to_string("src/downloads.json")?;
        let downloads_gold_json = std::fs::read_to_string("src/downloads_gold.json")?;
        let platforms_json = std::fs::read_to_string("src/platform.json")?;

        let downloads: File = serde_json::from_str(&downloads_json).unwrap();
        let downloads_gold: File = serde_json::from_str(&downloads_gold_json).unwrap();

        let platforms: Vec<PlatformInfo> = serde_json::from_str(&platforms_json).unwrap();

        // println!("{:#?}", downloads);
        // println!("{:#?}", platforms);

        // OK, here we need to preprocess the downloads together with the platform data.

        Ok(Self {
            app_version: "1.17".to_string(),
        })
    }
}

pub struct Config {
    pub indir: PathBuf,
    pub outdir: PathBuf,
    pub markdown_options: markdown::Options,
    pub global_meta: GlobalMeta,
}

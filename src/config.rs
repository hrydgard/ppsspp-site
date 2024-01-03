use std::{fmt::Binary, path::PathBuf};

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
    pub prod: bool,
}

impl GlobalMeta {
    pub fn new(production: bool) -> anyhow::Result<Self> {
        // Parse the download path dump.

        let downloads_json = std::fs::read_to_string("src/downloads.json")?;
        let downloads_gold_json = std::fs::read_to_string("src/downloads_gold.json")?;
        let platforms_json = std::fs::read_to_string("src/platform.json")?;

        let downloads: File = serde_json::from_str(&downloads_json).unwrap();
        let downloads_gold: File = serde_json::from_str(&downloads_gold_json).unwrap();

        let platforms: Vec<PlatformInfo> = serde_json::from_str(&platforms_json).unwrap();

        let files = parse_files(downloads, downloads_gold);

        println!("{:#?}", files);
        println!("{:#?}", platforms);

        // println!("{:#?}", downloads);
        // println!("{:#?}", platforms);

        // OK, here we need to preprocess the downloads together with the platform data.

        Ok(Self {
            app_version: "1.17".to_string(),
            prod: production,
        })
    }
}

#[derive(Debug)]
struct BinaryVersion {
    version: String,
    files: Vec<String>,
}

fn to_binaries_per_version(files: File) -> Vec<BinaryVersion> {
    files
        .children
        .iter()
        .filter(|child| !child.children.is_empty() && child.name.find('_').is_some())
        .map(|child| BinaryVersion {
            version: child.name.replace("_", "."),
            files: child
                .children
                .iter()
                .map(|subchild| subchild.name.clone())
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>()
}

fn parse_files(files: File, gold_files: File) -> Vec<BinaryVersion> {
    let mut binaries_per_version = to_binaries_per_version(files);

    binaries_per_version.sort_by(|a, b| natord::compare(&a.version, &b.version));

    let mut binaries_per_version_gold = to_binaries_per_version(gold_files);

    for b in &mut binaries_per_version {
        for g in &mut binaries_per_version_gold {
            if g.version == b.version {
                b.files.append(&mut g.files);
            }
        }
    }

    binaries_per_version
}

fn compute_gold_url_base(url_base: &str) -> String {
    format!("{}/api/goldfiles/", url_base)
}

pub struct Config {
    pub url_base: String,
    pub indir: PathBuf,
    pub outdir: PathBuf,
    pub markdown_options: markdown::Options,
    pub global_meta: GlobalMeta,
}

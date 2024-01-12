use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<File>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Author {
    name: String,
    url: String,
    image_url: String,
    title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DownloadInfo {
    name: String,
    title: Option<String>,
    icon: Option<String>,
    url: Option<String>,
    download_url: Option<String>,
    service_image: Option<String>,
    service_alt: Option<String>,
    short_name: Option<String>,
    whats_this: Option<String>,
    whats_this_url: Option<String>,
    filename: Option<String>,
    #[serde(default)]
    gold: bool,
    #[serde(default)]
    gold_only: bool,
    #[serde(default)]
    login_prompt: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PlatformInfo {
    title: String,
    platform_badge: Option<String>,
    platform_key: String, // we can ignore this, this was for react
    downloads: Vec<DownloadInfo>,
    #[serde(default)]
    newline: bool, // visual purposes only
}

#[derive(Debug)]
struct BinaryVersion {
    version: String,
    files: Vec<String>,
}

// Boiled-down version of the Previous Releases table for easy template consumption.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VersionDownloads {
    version: String,
    downloads: Vec<DownloadInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocLink {
    pub url: String,
    pub title: String,
    pub summary: Option<String>,
    #[serde(default)]
    pub external: bool,
    #[serde(default)]
    pub selected: bool,
}

impl DocLink {
    pub fn new(url: &str, title: &str, summary: Option<String>, selected_url: &str) -> Self {
        Self {
            url: url.to_string(),
            title: title.to_string(),
            summary,
            external: url.starts_with("https://"),
            selected: url == selected_url,
        }
    }
}

// Boiled-down version of the Previous Releases table for easy template consumption.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Screenshot {
    description: String,
    filename: String,
    #[serde(default)]
    index: i32, // 1-based, not read from file.
}
// This contains a bunch of stuff that various pages want to reference.
// Little point in restricting certain data to certain pages since we're a static generator
// so it'll be a grab bag of stuff. If we get performance problems one day, we'll split this up.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GlobalMeta {
    pub app_version: String,
    pub platforms: Vec<PlatformInfo>,
    pub version_downloads: Vec<VersionDownloads>,
    pub top_nav: Vec<DocLink>,
    pub prod: bool,
    pub authors: HashMap<String, Author>,
    pub screenshots: Vec<Screenshot>,
}

fn download_path(url_base: &str, version: &str, filename: &str) -> String {
    format!(
        "{}/files/{}/{}",
        url_base,
        version.replace('.', "_"),
        filename
    )
}

fn gold_download_path(url_base: &str, version: &str, filename: &str) -> String {
    format!(
        "{}/api/goldfiles/{}/{}",
        url_base,
        version.replace('.', "_"),
        filename
    )
}

impl GlobalMeta {
    pub fn new(production: bool, url_base: &str, top_nav: Vec<DocLink>) -> anyhow::Result<Self> {
        // Parse the download path dump.

        let downloads_json = std::fs::read_to_string("data/downloads.json")?;
        let downloads_gold_json = std::fs::read_to_string("data/downloads_gold.json")?;
        let platforms_json = std::fs::read_to_string("data/platform.json")?;
        let authors_json = std::fs::read_to_string("data/authors.json")?;
        let screenshots_json = std::fs::read_to_string("data/screenshots.json")?;

        let downloads: File = serde_json::from_str(&downloads_json).unwrap();
        let downloads_gold: File = serde_json::from_str(&downloads_gold_json).unwrap();
        let authors: HashMap<String, Author> = serde_json::from_str(&authors_json).unwrap();
        let mut screenshots: Vec<Screenshot> = serde_json::from_str(&screenshots_json).unwrap();
        for (index, shot) in screenshots.iter_mut().enumerate() {
            shot.index = (index as i32) + 1;
        }

        let mut platforms: Vec<PlatformInfo> = serde_json::from_str(&platforms_json).unwrap();

        let version_binaries = parse_files(downloads, downloads_gold);

        let file_versions = pivot(&version_binaries);

        // Update the platforms with URLs
        for (index, platform) in &mut platforms.iter_mut().enumerate() {
            for download in &mut platform.downloads {
                if let Some(filename) = &download.filename {
                    if let Some(version) = file_versions.get(filename) {
                        if let Some(first) = version.first() {
                            download.download_url = if download.gold_only {
                                Some(gold_download_path(url_base, first, filename))
                            } else {
                                Some(download_path(url_base, first, filename))
                            };
                        }
                    }
                }
            }
            if (index % 3) == 2 {
                platform.newline = true;
            }
        }

        let version_downloads = boil(&version_binaries, &platforms);

        //println!("{:#?}", version_binaries);
        //println!("{:#?}", file_versions);
        //println!("{:#?}", platforms);
        //println!("{:#?}", version_downloads);

        // println!("{:#?}", downloads);
        // println!("{:#?}", platforms);
        // println!("{:#?}", authors);

        // OK, here we need to preprocess the downloads together with the platform data.

        Ok(Self {
            app_version: if let Some(file) = version_binaries.first() {
                file.version.clone()
            } else {
                "indeterminate".to_string()
            },
            authors,
            prod: production,
            platforms,
            version_downloads,
            top_nav,
            screenshots,
        })
    }
}

fn to_binaries_per_version(files: File) -> Vec<BinaryVersion> {
    files
        .children
        .iter()
        .filter(|child| !child.children.is_empty() && child.name.find('_').is_some())
        .map(|child| BinaryVersion {
            version: child.name.replace('_', "."),
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

    // reverse order, newest first
    binaries_per_version.sort_by(|a, b| natord::compare(&b.version, &a.version));

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

fn pivot(binaries_per_version: &Vec<BinaryVersion>) -> HashMap<String, Vec<String>> {
    let mut hash = HashMap::<String, Vec<String>>::new();
    for entry in binaries_per_version {
        for name in &entry.files {
            hash.entry(name.clone())
                .or_default()
                .push(entry.version.clone());
        }
    }
    hash
}

fn boil(version_binaries: &[BinaryVersion], platforms: &[PlatformInfo]) -> Vec<VersionDownloads> {
    let mut downloads = vec![];
    for version in version_binaries {
        let mut version_download = VersionDownloads {
            version: version.version.clone(),
            downloads: vec![],
        };

        for platform in platforms {
            let mut first = true;
            for filename in &version.files {
                //println!("filename: {}", filename);
                //println!("{:#?}", platform.downloads);
                // Look up the filename in platforms, the ugly way.
                if let Some(download) = platform
                    .downloads
                    .iter()
                    .find(|download| download.filename.as_ref() == Some(filename))
                {
                    // Add this download!
                    let mut download = download.clone();
                    if first {
                        download.icon = platform.platform_badge.clone();
                        first = false;
                    } else {
                        download.icon = None;
                    }
                    version_download.downloads.push(download);
                }
            }
        }
        if !version_download.downloads.is_empty() {
            downloads.push(version_download);
        }
    }
    downloads
}

pub struct Config {
    pub url_base: String,
    pub in_dir: PathBuf,
    pub out_dir: PathBuf,
    pub markdown_options: markdown::Options,
    pub global_meta: GlobalMeta,
    pub build_date: String,
    pub github_url: &'static str,
}

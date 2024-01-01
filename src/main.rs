// Things to generate:
// - [...] Docs
// - [ ] Pages
//   - [ ] Downloads
//   - [ ] Regular static pages
// - [ ] Blog
// - [ ] News

use std::{path::{Path, PathBuf}, io::Write};

extern crate anyhow;
pub mod config;
pub mod util;
use anyhow::Context;
pub use config::Config;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CategoryMeta {
    #[serde(rename = "label")]
    pub title: String,
    pub position: u32,
    pub link: Option<Link>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub html: String,
    pub title: String,
}

impl Document {
    pub fn from_md(md_path: &Path) -> anyhow::Result<Document> {
        let md = std::fs::read_to_string(&md_path)?;
        let html = markdown::to_html_with_options(&md, &markdown::Options::gfm())
            .map_err(anyhow::Error::msg)?;
        let title = util::title_string(&md_path); // TODO: Only open the file once. But who cares, really.
        Ok(Self {
            path: md_path.to_path_buf(),
            html,
            title,
        })
    }
}

#[derive(Debug)]
struct Category {
    meta: CategoryMeta,
    documents: Vec<Document>,
    sub_categories: Vec<Category>,
}

impl Category {
    pub fn from_folder_tree(folder: &Path) -> anyhow::Result<Self> {
        let mut documents = vec![];
        let mut sub_categories = vec![];
        let listing = folder.read_dir()?;
        let mut meta = CategoryMeta::default();
        meta.title = "Untitled".to_string();

        for dir_entry in listing {
            let entry = dir_entry?;
            let path = folder.join(entry.file_name());
            let name = entry.file_name().to_str().unwrap().to_owned();
            if entry.metadata()?.is_dir() {
                sub_categories.push(Self::from_folder_tree(&path)?);
            } else {
                // Check file extension to figure out what to do.
                if let Some(os_str) = path.extension() {
                    match os_str.to_str() {
                        Some("md") => {
                            documents.push(Document::from_md(&path)?);
                        }
                        Some("json") => {
                            if name == "_category_.json" {
                                let json_str = std::fs::read_to_string(path)?;
                                meta =
                                    serde_json::from_str(&json_str).expect("Failed to parse JSON");
                            }
                        }
                        _ => {
                            anyhow::bail!("Unhandled file type: {}", path.display());
                        }
                    }
                }
            }
        }

        Ok(Self {
            meta,
            documents,
            sub_categories,
        })
    }

    fn recurse(&self) -> Vec<Document> {
        let mut all_docs = vec![];
        for doc in &self.documents {
            all_docs.push(doc.clone());
        }
        for cat in &self.sub_categories {
            all_docs.extend(cat.recurse());
        }
        all_docs

    }

    pub fn all_documents(&self) -> Vec<Document> {
        self.recurse()
    }
}

fn generate_doctree(config: &Config, folder: &str) -> anyhow::Result<()> {
    // First, build the tree and convert all the markdown to html and metadata.
    let root_folder = config.indir.join(folder);
    let out_root_folder = config.outdir.clone();
    let root_cat = Category::from_folder_tree(&root_folder)?;

    // Write out all the files. Don't need recursion here so we can linearize.
    let docs = root_cat.all_documents();

    for doc in docs {
        let mut target_path = out_root_folder.join(doc.path);
        target_path.set_extension("");

        util::create_folder_if_missing(&target_path);

        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(doc.html.as_bytes())?;
    }

    // MD documents get wrapped into our doc templat&e.
    // println!("{:#?}", root_cat);

    Ok(())
}

fn generate_blog(config: &Config, folder: &str) {
    // For the blog

    let root = config.indir.join(folder);
    let out_root = config.outdir.join(folder);
}

fn generate_pages(config: &Config, folder: &str) -> anyhow::Result<()> {
    let root_folder = config.indir.join(folder);
    // pages are generated directly into the root.
    let out_root_folder = &config.outdir;

    let listing = root_folder.read_dir()?;
    for entry in listing {
        let entry = entry?;
        let html_path = root_folder.join(entry.file_name());
        let name = entry.file_name().to_str().unwrap().to_owned();

        // TODO: Allow md files here too!
        // TODO: Parse metadata out of the files somehow!
        let html = std::fs::read_to_string(&html_path)?;
        let mut target_path = out_root_folder.join(name);

        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(html.as_bytes())?;
    }
    Ok(())
}

fn main() {
    println!("Barebones website generator");

    let config = Config {
        indir: PathBuf::from("."),
        outdir: PathBuf::from("build"),
    };

    if !config.outdir.exists() {
        std::fs::create_dir(&config.outdir).context("outdir").unwrap();
    }

    generate_pages(&config, "src/pages").unwrap();

    generate_blog(&config, "blog");
    generate_blog(&config, "news");

    generate_doctree(&config, "docs").unwrap();
}

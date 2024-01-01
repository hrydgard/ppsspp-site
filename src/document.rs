use crate::util;
use std::{
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

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

#[derive(Debug, Clone, Default)]
pub struct DocumentMeta {
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub html: String,
    pub meta: DocumentMeta,
}

impl Document {
    fn read_dash_meta(reader: &mut impl BufRead) -> DocumentMeta {
        let mut meta = DocumentMeta::default();
        let mut buffer = String::new();

        reader.read_line(&mut buffer);

        if !buffer.starts_with("---") {
            // Won't be anything to read.
            return meta;
        }

        let mut found_end = false;
        buffer.clear();
        while let Ok(_) = reader.read_line(&mut buffer) {
            if buffer.starts_with("---") {
                found_end = true;
                break;
            }
            if let Some((key, value)) = buffer.split_once(':') {
                match key {
                    "title" => meta.title = value.to_owned(),
                    _ => {}
                }
            }

            buffer.clear();
        }

        // TODO: runtime error
        assert!(found_end);
        return meta;
    }

    pub fn from_md(md_path: &Path, options: &markdown::Options) -> anyhow::Result<Document> {
        let md_file = std::fs::File::open(&md_path)?;
        let mut reader = BufReader::new(md_file);

        let mut meta = Self::read_dash_meta(&mut reader);
        // If no dash-meta, grab the title string.
        if meta.title.is_empty() {
            meta.title = util::title_string(&mut reader); // TODO: Only open the file once. But who cares, really.
        }

        // OK, read the rest.
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;

        let md = String::from_utf8(buffer)?;

        let html = markdown::to_html_with_options(&md, options).map_err(anyhow::Error::msg)?;
        Ok(Self {
            path: md_path.to_path_buf(),
            html,
            meta,
        })
    }
    pub fn from_html(html_path: &Path) -> anyhow::Result<Document> {
        let html = std::fs::read_to_string(&html_path)?;
        Ok(Self {
            path: html_path.to_path_buf(),
            html,
            meta: DocumentMeta {
                title: "untitled html".to_string(),
            },
        })
    }
}

#[derive(Debug)]
pub struct Category {
    pub meta: CategoryMeta,
    pub documents: Vec<Document>,
    pub sub_categories: Vec<Category>,
}

impl Category {
    pub fn from_folder_tree(folder: &Path, options: &markdown::Options) -> anyhow::Result<Self> {
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
                sub_categories.push(Self::from_folder_tree(&path, options)?);
            } else if let Some(os_str) = path.extension() {
                // Check file extension to figure out what to do.
                match os_str.to_str().unwrap() {
                    "md" => {
                        documents.push(Document::from_md(&path, options)?);
                    }
                    "json" => {
                        if name == "_category_.json" {
                            let json_str = std::fs::read_to_string(path)?;
                            meta = serde_json::from_str(&json_str).expect("Failed to parse JSON");
                        }
                    }
                    _ => {
                        anyhow::bail!("Unhandled file type: {}", path.display());
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

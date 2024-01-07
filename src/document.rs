use crate::util;
use std::{
    ffi::OsString,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

extern crate serde;
use serde::{Deserialize, Serialize};

use crate::config::{DocLink, GlobalMeta};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "type")]
    pub link_type: String,
    pub description: String,
}

// This is passed into rendering of blog posts, for example,
// as well as being used in the rust code.
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DocumentMeta {
    #[serde(default)]
    pub position: u32,
    #[serde(rename = "label")]
    pub title: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub link: Option<Link>,
    #[serde(default)]
    pub url: String,
    pub prev: Option<DocLink>,
    pub next: Option<DocLink>,
    #[serde(default)]
    pub contains_code: bool,
    #[serde(default)]
    pub breadcrumbs: Vec<DocLink>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf, // written file, the link-to path is in meta
    pub html: String,
    pub meta: DocumentMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SidebarContext {
    pub title: String,
    pub links: Vec<DocLink>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub name: String,
    pub articles: Vec<DocLink>,
}

// Used when rendering templates.
// Should probably split into multiple more focused ones .. but then again, not really necessary,
// can just omit what we don't need.
#[derive(Serialize, Deserialize)]
pub struct PageContext {
    pub title: Option<String>,
    pub contents: Option<String>,
    pub sidebar: Option<String>,
    pub children: Vec<DocLink>,
    pub meta: Option<DocumentMeta>,
    pub year: i32,
    pub globals: Option<GlobalMeta>,
    pub tags: Vec<Tag>,
    pub contains_code: bool,
}

impl PageContext {
    pub fn new(title: Option<String>, contents: Option<String>, globals: &GlobalMeta) -> Self {
        Self {
            title,
            contents,
            sidebar: None,
            year: 2024,
            children: vec![],
            meta: None,
            globals: Some(globals.clone()),
            tags: vec![],
            contains_code: false,
        }
    }
    pub fn from_document(document: &Document, globals: &GlobalMeta) -> Self {
        Self {
            title: Some(document.meta.title.clone()),
            contents: Some(document.html.clone()),
            sidebar: None,
            year: 2024,
            children: vec![],
            meta: Some(document.meta.clone()),
            globals: Some(globals.clone()),
            tags: vec![],
            contains_code: document.meta.contains_code,
        }
    }
    pub fn render(
        mut self,
        template_name: &str,
        handlebars: &mut handlebars::Handlebars,
    ) -> anyhow::Result<String> {
        self.update_selected();
        Ok(handlebars.render(template_name, &self)?)
    }
    pub fn render_template(
        mut self,
        template_string: &str,
        handlebars: &mut handlebars::Handlebars,
    ) -> anyhow::Result<String> {
        self.update_selected();
        Ok(handlebars.render_template(template_string, &self)?)
    }

    fn update_selected(&mut self) {
        if let Some(ref mut globals) = &mut self.globals {
            if let Some(meta) = &self.meta {
                let self_url = &meta.url;
                for link in &mut globals.top_nav {
                    if self_url.starts_with(&link.url) {
                        link.selected = true;
                    }
                }
            }
        }
    }
}

fn postprocess_markdown(md: String) -> String {
    md.replace("<table>", "<table class=\"ms-table\">")
}

fn split_bracketed_list(value: &str) -> Vec<String> {
    if let Some(value) = value.strip_prefix('[') {
        if let Some(value) = value.strip_suffix(']') {
            return value
                .split(',')
                .map(|s| s.trim().to_owned())
                .collect::<Vec<_>>();
        }
    }
    vec![]
}

fn cleanup_path(path: &Path) -> Option<String> {
    path.to_string_lossy()
        .strip_prefix('.')
        .map(|s| s.to_string().replace('\\', "/"))
}

pub fn strip_extension(str: OsString) -> String {
    let mut x = PathBuf::from(str);
    x.set_extension("");
    x.to_string_lossy().to_string()
}

impl Document {
    pub fn to_doclink(&self, selected_url: &str) -> DocLink {
        DocLink::new(&self.meta.url.clone(), &self.meta.title, None, selected_url)
    }

    fn read_dash_meta(reader: &mut impl BufRead) -> anyhow::Result<(DocumentMeta, bool)> {
        let mut meta = DocumentMeta::default();
        let mut buffer = String::new();

        reader.read_line(&mut buffer)?;

        if !buffer.starts_with("---") {
            // Won't be anything to read.
            return if let Some(suffix) = buffer.strip_prefix("# ") {
                meta.title = suffix.trim().to_string();
                Ok((meta, true))
            } else {
                Ok((meta, false))
            };
        }

        let mut found_end = false;
        buffer.clear();
        while reader.read_line(&mut buffer).is_ok() {
            if buffer.starts_with("---") {
                found_end = true;
                break;
            }
            if let Some((key, value)) = buffer.split_once(':') {
                let value = value.trim().to_owned();
                match key {
                    "title" => meta.title = value,
                    "slug" => meta.slug = value,
                    "authors" => meta.author = value,
                    "tags" => meta.tags = split_bracketed_list(&value),
                    _ => {}
                }
            }

            buffer.clear();
        }

        // TODO: runtime error
        assert!(found_end);
        Ok((meta, false))
    }

    // Handles page, blog posts, etc, including triple-dash docusaurus-style metadata.
    pub fn from_md(md_path: &Path, options: &markdown::Options) -> anyhow::Result<Self> {
        let md_file = std::fs::File::open(md_path)?;
        let mut reader = BufReader::new(md_file);
        let (mut meta, mut ate_title) = Self::read_dash_meta(&mut reader)?;

        let mut path = md_path.to_path_buf();
        path.set_extension("");

        meta.url = cleanup_path(&path).unwrap();
        //if let Some(name) = name {
        //    meta.url = format!("/{name}");
        //}

        let mut md = String::from("");

        let github_url = "https://github.com/hrydgard/ppsspp/issues/";

        // If no dash-meta, grab the title string.
        if meta.title.is_empty() {
            let mut buffer = String::new();
            let _ = reader.read_line(&mut buffer);
            if let Some(remaining) = buffer.strip_prefix("# ") {
                meta.title = remaining.to_string(); // TODO: Only open the file once. But who cares, really.
                ate_title = true;
            }
        }
        if ate_title {
            md += &format!("# {}\n", meta.title);
        }

        // OK, read the rest.
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;

        md += &String::from_utf8(buffer)?;

        if md.contains("```c") || md.contains("```rust") {
            meta.contains_code = true;
        }

        let issue_regex = regex::Regex::new(r"\[#(\d+)\]").unwrap();
        md = issue_regex
            .replace_all(&md, |captures: &regex::Captures| {
                let issue_number = captures.get(1).unwrap().as_str();
                format!("[#{}]({}{})", issue_number, github_url, issue_number) // Construct the replacement with the GitHub URL
            })
            .to_string();

        let html = markdown::to_html_with_options(&md, options).map_err(anyhow::Error::msg)?;
        let html = postprocess_markdown(html);

        Ok(Self { path, html, meta })
    }

    // The document itself is the template so we apply it immediately. Used for pages.
    pub fn from_hbs(
        globals: &GlobalMeta,
        name: &str,
        hbs_path: &Path,
        handlebars: &mut handlebars::Handlebars,
    ) -> anyhow::Result<Self> {
        let hbs = std::fs::read_to_string(hbs_path)?;
        let mut context = PageContext::new(None, None, globals);
        context.globals = Some(globals.clone());
        let meta = DocumentMeta {
            url: format!("/{name}"),
            ..Default::default()
        };
        context.meta = Some(meta.clone());
        let html = context.render_template(&hbs, handlebars)?;
        Ok(Self {
            path: hbs_path.to_path_buf(),
            meta,
            html,
        })
    }

    // Applies the "doc" template.
    pub fn from_html(html_path: &Path) -> anyhow::Result<Self> {
        let html = std::fs::read_to_string(html_path)?;
        Ok(Self {
            path: html_path.to_path_buf(),
            html,
            meta: DocumentMeta {
                title: "untitled html".to_string(),
                ..Default::default()
            },
        })
    }

    // Category front pages.
    pub fn from_category(
        category: &Category,
        handlebars: &mut handlebars::Handlebars,
        globals: &GlobalMeta,
    ) -> anyhow::Result<Self> {
        let mut context = PageContext::new(Some(category.meta.title.clone()), None, globals);

        context.children = category
            .documents
            .iter()
            .map(|doc| doc.to_doclink(&category.meta.url))
            .collect::<Vec<_>>();

        // Add children document titles to the context here.

        let html = handlebars.render("cat_contents", &context)?;
        Ok(Self {
            path: category.path.clone(),
            html,
            meta: category.meta.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub meta: DocumentMeta,
    pub documents: Vec<Document>,
    pub sub_categories: Vec<Category>,
    pub path: PathBuf,
}

impl Category {
    pub fn from_folder_tree(folder: &Path, options: &markdown::Options) -> anyhow::Result<Self> {
        let mut documents = vec![];
        let mut sub_categories = vec![];
        let listing = folder.read_dir()?;
        let mut meta = DocumentMeta {
            title: "Documentation".to_string(),
            url: "/docs".to_string(),
            ..Default::default()
        };

        for dir_entry in listing {
            let entry = dir_entry?;
            let path = folder.join(entry.file_name());
            let name = util::filename_to_string(&entry.file_name());

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

        let path = folder.to_path_buf();
        meta.url = cleanup_path(&path).unwrap();
        Ok(Self {
            meta,
            documents,
            sub_categories,
            path,
        })
    }

    // Note: This also generates category documents.
    pub fn all_documents(
        &self,
        handlebars: &mut handlebars::Handlebars,
        globals: &GlobalMeta,
    ) -> anyhow::Result<Vec<Document>> {
        let mut all_docs = vec![];
        all_docs.push(Document::from_category(self, handlebars, globals)?);
        for doc in &self.documents {
            all_docs.push(doc.clone());
        }
        for cat in &self.sub_categories {
            all_docs.extend(cat.all_documents(handlebars, globals)?);
        }
        // Add next/forward links
        // for [prev, cur, next] in documents.
        for i in 0..all_docs.len() {
            if let Some(prev) = all_docs.get(i.wrapping_sub(1)) {
                all_docs[i].meta.prev = Some(prev.to_doclink(""));
            }
            if let Some(next) = all_docs.get(i.wrapping_add(1)) {
                all_docs[i].meta.next = Some(next.to_doclink(""));
            }
        }
        Ok(all_docs)
    }

    pub fn to_doclink(&self) -> DocLink {
        DocLink {
            url: self.meta.url.clone(),
            title: self.meta.title.clone(),
            summary: None,
            external: false,
            selected: false,
        }
    }

    pub fn compute_breadcrumbs(&mut self, crumbs: &mut Vec<DocLink>) {
        self.meta.breadcrumbs = crumbs.clone();
        for doc in &mut self.documents {
            crumbs.push(doc.to_doclink(""));
            doc.meta.breadcrumbs = crumbs.clone();
            crumbs.pop();
        }
        for cat in &mut self.sub_categories {
            crumbs.push(cat.to_doclink());
            cat.compute_breadcrumbs(crumbs);
            crumbs.pop();
        }
    }
}

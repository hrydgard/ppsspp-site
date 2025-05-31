use crate::{
    config::{Config, DocLink, GlobalMeta},
    post_process, util,
};
use std::{
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use serde::Serialize;

// This is passed into rendering of blog posts, for example,
// as well as being used in the rust code. And being deserialized
// from _category.json_, which we should probably replace.
#[derive(Debug, Serialize, Clone, Default)]
pub struct DocumentMeta {
    #[serde(default)]
    pub position: u32,
    #[serde(rename = "label")]
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub url: String,
    pub prev: Option<DocLink>,
    pub next: Option<DocLink>,
    #[serde(default)]
    pub contains_code: bool,
    #[serde(default)]
    pub breadcrumbs: Vec<DocLink>,
    #[serde(default)]
    pub section: String,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf, // written file, the link-to path is in meta
    pub markdown: Option<String>,
    pub html: String,
    pub meta: DocumentMeta,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct SidebarContext {
    pub title: String,
    pub links: Vec<DocLink>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Tag {
    pub name: String,
    pub articles: Vec<DocLink>,
    pub selected: bool,
}

// Used when rendering templates.
// Should probably split into multiple more focused ones .. but then again, not really necessary,
// can just omit what we don't need.
#[derive(Serialize)]
pub struct PageContext<'a> {
    pub title: Option<String>,
    pub contents: Option<String>,
    pub sidebar: Option<String>,
    pub children: Vec<DocLink>,
    pub meta: Option<DocumentMeta>,
    pub globals: Option<&'a GlobalMeta>,
    pub tags: &'a [Tag],
    pub contains_code: bool,
    pub top_nav: Vec<DocLink>,
}

impl<'a> PageContext<'a> {
    pub fn new(title: Option<String>, contents: Option<String>, globals: &'a GlobalMeta) -> Self {
        Self {
            title,
            contents,
            sidebar: None,
            children: vec![],
            meta: None,
            globals: Some(globals),
            tags: &[],
            contains_code: false,
            top_nav: globals.top_nav.clone(),
        }
    }
    pub fn from_document(document: &Document, globals: &'a GlobalMeta) -> Self {
        Self {
            title: Some(document.meta.title.clone()),
            contents: Some(document.html.clone()),
            sidebar: None,
            children: vec![],
            meta: Some(document.meta.clone()),
            globals: Some(globals),
            tags: &[],
            contains_code: document.meta.contains_code,
            top_nav: globals.top_nav.clone(),
        }
    }
    pub fn render(
        mut self,
        template_name: &str,
        handlebars: &mut handlebars::Handlebars<'_>,
    ) -> anyhow::Result<String> {
        self.update_selected();
        Ok(handlebars.render(template_name, &self)?)
    }
    pub fn render_template(
        mut self,
        template_string: &str,
        handlebars: &mut handlebars::Handlebars<'_>,
    ) -> anyhow::Result<String> {
        self.update_selected();
        Ok(handlebars.render_template(template_string, &self)?)
    }

    fn update_selected(&mut self) {
        if let Some(meta) = &self.meta {
            let self_url = &meta.url;
            for link in &mut self.top_nav {
                if self_url.starts_with(&link.url) {
                    link.selected = true;
                }
            }
        }
    }
}

fn postprocess_html(md: String) -> String {
    md.replace("<table>", "<table class=\"nice-table\">")
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

impl Document {
    pub fn to_doclink(&self, selected_url: &str) -> DocLink {
        DocLink::new(
            &self.meta.url.clone(),
            &self.meta.title,
            &self.meta.date,
            self.meta.summary.clone(),
            selected_url,
        )
    }

    pub fn read_dash_meta(reader: &mut impl BufRead) -> anyhow::Result<(DocumentMeta, bool)> {
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
                    "position" => meta.position = str::parse(&value).unwrap_or_default(),
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
    pub fn from_md(md_path: &Path, config: &Config) -> anyhow::Result<Self> {
        let md_file = std::fs::File::open(md_path)?;
        let mut reader = BufReader::new(md_file);
        let (mut meta, ate_title) = Self::read_dash_meta(&mut reader)?;

        let mut path = md_path.to_path_buf();
        path.set_extension("");

        meta.url = cleanup_path(&path).unwrap();

        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;

        let mut md = String::new();
        if ate_title {
            md += &format!("# {}\n", meta.title);
        }

        md += &String::from_utf8(buffer)?;

        post_process::add_meta_from_markdown(&md, &mut meta, config)?;

        if md.contains("```") {
            meta.contains_code = true;
        }

        let md = post_process::preprocess_markdown(&md, &meta.title, config)?;

        let html = markdown::to_html_with_options(&md, &config.markdown_options)
            .map_err(anyhow::Error::msg)?;
        let html = postprocess_html(html);

        Ok(Self {
            path,
            markdown: Some(md),
            html,
            meta,
        })
    }

    // The document itself is the template so we apply it immediately. Used for pages.
    pub fn from_hbs(
        globals: &GlobalMeta,
        name: &str,
        hbs_path: &Path,
        handlebars: &mut handlebars::Handlebars<'_>,
    ) -> anyhow::Result<Self> {
        let hbs = std::fs::read_to_string(hbs_path)?;
        let mut context = PageContext::new(None, None, globals);
        context.globals = Some(globals);
        let meta = DocumentMeta {
            url: format!("/{name}"),
            ..Default::default()
        };
        context.meta = Some(meta.clone());
        let html = context.render_template(&hbs, handlebars)?;
        Ok(Self {
            path: hbs_path.to_path_buf(),
            markdown: None,
            meta,
            html,
        })
    }

    // Applies the "doc" template.
    pub fn from_html(html_path: &Path) -> anyhow::Result<Self> {
        let html = std::fs::read_to_string(html_path)?;
        Ok(Self {
            path: html_path.to_path_buf(),
            markdown: None,
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
        handlebars: &mut handlebars::Handlebars<'_>,
        globals: &GlobalMeta,
    ) -> anyhow::Result<Self> {
        let mut context = PageContext::new(Some(category.meta.title.clone()), None, globals);

        context.children = category
            .documents
            .iter()
            .map(|doc| doc.to_doclink(&category.meta.url))
            .collect::<Vec<_>>();

        context
            .children
            .extend(category.sub_categories.iter().map(|cat| cat.to_doclink()));

        context.contents = Some(category.html.clone());

        // Add children document titles to the context here.

        let html = handlebars.render("cat_contents", &context)?;
        Ok(Self {
            path: category.path.clone(),
            html,
            markdown: None,
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
    pub html: String,
}

fn add_positions(crumbs: &mut [DocLink]) {
    for (i, crumb) in crumbs.iter_mut().enumerate() {
        crumb.position = i + 1;
    }
}

impl Category {
    pub fn from_folder_tree(folder: &Path, config: &Config) -> anyhow::Result<Self> {
        let mut documents = vec![];
        let mut sub_categories = vec![];
        let listing = folder.read_dir()?;
        let mut meta = DocumentMeta {
            title: "Documentation".to_string(),
            url: "/docs".to_string(),
            ..Default::default()
        };

        let mut summary = "<ul>".to_string();
        let mut summary_line_count = 0;

        let mut html = "".to_string();

        const MAX_SUMMARY_LINES: usize = 3;

        for dir_entry in listing {
            let entry = dir_entry?;
            let path = folder.join(entry.file_name());
            let name = util::filename_to_string(&entry.file_name());

            if entry.metadata()?.is_dir() {
                sub_categories.push(Self::from_folder_tree(&path, config)?);
            } else if let Some(os_str) = path.extension() {
                // Check file extension to figure out what to do.
                match os_str.to_str().unwrap() {
                    "md" => {
                        match Document::from_md(&path, config) {
                            Ok(doc) => {
                                if name == "_category_.md" {
                                    meta = doc.meta.clone();
                                    html = doc.html;
                                    // let mut _ate_title;
                                    //(meta, _ate_title) = Document::read_dash_meta(&mut reader)?;
                                } else {
                                    documents.push(doc);
                                }
                            }
                            Err(err) => anyhow::bail!("Failed to parse {}: {}", path.display(), err.to_string()),
                        }
                    }
                    _ => {
                        anyhow::bail!("Unhandled file type: {}", path.display());
                    }
                }
            }
        }

        // First sort alphabetically, then by key.
        sub_categories.sort_by_cached_key(|a| a.meta.title.clone());
        documents.sort_by_cached_key(|a| a.meta.title.clone());

        // Best-effort sorting using the given numbers. Numbers are ahead of no number.
        sub_categories.sort_by_key(|a| {
            if a.meta.position == 0 {
                1000
            } else {
                a.meta.position
            }
        });

        documents.sort_by_key(|d| {
            if d.meta.position == 0 {
                1000
            } else {
                d.meta.position
            }
        });

        // Compute the summaries after sorting.
        for cat in &sub_categories {
            if summary_line_count < MAX_SUMMARY_LINES {
                summary += &format!("<li><strong>{}</strong></li>", &cat.meta.title);
            }
            summary_line_count += 1;
        }
        for doc in &documents {
            if summary_line_count < MAX_SUMMARY_LINES {
                summary += &format!("<li>{}</li>", &doc.meta.title);
            }
            summary_line_count += 1;
        }

        if summary_line_count >= 4 {
            summary += "<li>...</li>";
        }
        summary += "</ul>";

        let path = folder.to_path_buf();
        meta.url = cleanup_path(&path).unwrap();
        meta.summary = Some(summary);

        Ok(Self {
            meta,
            documents,
            sub_categories,
            path,
            html,
        })
    }

    // Note: This also generates category documents.
    pub fn all_documents(
        &self,
        handlebars: &mut handlebars::Handlebars<'_>,
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
        Ok(all_docs)
    }

    pub fn add_prev_next_links(all_docs: &mut [Document]) {
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
    }

    pub fn to_doclink(&self) -> DocLink {
        DocLink {
            url: self.meta.url.clone(),
            title: self.meta.title.clone(),
            date: self.meta.date.clone(),
            summary: self.meta.summary.clone(),
            external: false,
            selected: false,
            position: 0,
        }
    }

    pub fn compute_breadcrumbs(&mut self, crumbs: &mut Vec<DocLink>) {
        self.meta.breadcrumbs = crumbs.clone();
        add_positions(&mut self.meta.breadcrumbs);

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

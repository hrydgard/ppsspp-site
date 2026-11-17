use crate::config::Config;
use crate::document::Document;
use crate::util;
use anyhow::Context;
use serde::Serialize;
use std::io::Write;

// changefreq and priority are left out: Google ignores them. lastmod is only known for posts.
#[derive(Clone, Serialize)]
struct SitemapEntry {
    loc: String,     // https://www.ppsspp.org/news/foo/
    lastmod: String, // 2005-01-01, or empty
}

#[derive(Serialize)]
pub struct SitemapGenerator {
    entries: Vec<SitemapEntry>,
}

impl SitemapGenerator {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }
    pub fn add_url(&mut self, url: &str, lastmod: &str) {
        self.entries.push(SitemapEntry {
            loc: format!("{}{}", util::SITE_URL, util::canonical_path(url)),
            lastmod: lastmod.to_string(),
        });
    }
    pub fn add(&mut self, documents: &[Document]) {
        for doc in documents {
            self.add_url(&doc.meta.url, &doc.meta.date);
        }
    }
    pub fn generate(
        &self,
        config: &Config,
        handlebars: &mut handlebars::Handlebars<'_>,
    ) -> anyhow::Result<()> {
        let xml = handlebars.render("sitemap_xml", &self)?;
        let target_path = config.out_dir.join("sitemap.xml");
        let mut file = std::fs::File::create(target_path).context("sitemap.xml")?;
        file.write_all(xml.as_bytes())?;
        Ok(())
    }
}

use crate::config::Config;
use crate::document::Document;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct SitemapEntry {
    loc: String,              // https://www.ppsspp.org/
    lastmod: String,          // 2005-01-01
    changefreq: &'static str, // "monthly"
    priority: String,         // 0.8
}

#[derive(Serialize)]
pub struct SitemapGenerator {
    entries: Vec<SitemapEntry>,
}

impl SitemapGenerator {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }
    pub fn add(&mut self, documents: &[Document], priority: f32) {
        for doc in documents {
            self.entries.push(SitemapEntry {
                loc: format!("https://www.ppsspp.org{}", doc.meta.url),
                lastmod: doc.meta.date.clone(),
                changefreq: "daily",
                priority: format!("{:.2}", priority),
            });
        }
    }
    pub fn generate(
        &self,
        _config: &Config,
        _handlebars: &mut handlebars::Handlebars<'_>,
    ) -> anyhow::Result<()> {
        //use anyhow::Context;
        // use std::io::Write;
        // let xml = handlebars.render("sitemap_xml", &self)?;
        // let target_path = config.out_dir.join("sitemap.xml");
        // let mut file = std::fs::File::create(target_path).context("sitemap.xml")?;
        // file.write_all(xml.as_bytes())?;
        Ok(())
    }
}

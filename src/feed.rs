use std::io::Write;

use crate::config::Config;
use crate::document::Document;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

use anyhow::Context;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Item {
    title: String,
    link: String,
    pubDate: String,
    category: String,
    description: String,
    tags: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Channel {
    title: String,
    link: String,
    link_folder: String,
    description: String,
    lastBuildDate: String,
    docs: String,
    language: String,
    items: Vec<Item>,
}

#[derive(Serialize)]
struct Rss {
    version: String,
    channel: Channel,
}

pub fn format_time(yyyy_mm_dd: &str, format: FeedFormat) -> String {
    let naive_date = NaiveDate::parse_from_str(yyyy_mm_dd, "%Y-%m-%d").unwrap();
    let naive_datetime = NaiveDateTime::new(naive_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    // Convert the NaiveDateTime to Utc
    let utc_datetime = Utc.from_utc_datetime(&naive_datetime);
    // Format the Utc DateTime in the desired format
    match format {
        FeedFormat::RSS => utc_datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
        FeedFormat::Atom => utc_datetime.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
pub enum FeedFormat {
    RSS,
    Atom,
}

// Posts should be passed-in in reverse time order.
pub fn write_feed(
    config: &Config,
    title: &str,
    description: &str,
    folder: &str,
    all_posts: &[Document],
    format: FeedFormat,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<()> {
    let base_path = format!("https://www.ppssspp.org/{folder}");
    let rss = Rss {
        version: "2.0".to_string(),
        channel: Channel {
            title: title.to_string(),
            link: format!("{base_path}/rss.xml"),
            link_folder: base_path.clone(),
            description: description.to_string(),
            lastBuildDate: config.build_date.clone(),
            docs: "https://validator.w3.org/feed/docs/rss2.html".to_owned(),
            language: "en".to_owned(),
            items: all_posts
                .iter()
                .map(|x| Item {
                    title: x.meta.title.clone(),
                    link: format!("{base_path}{}", x.meta.url),
                    description: x.meta.title.clone(),
                    category: x.meta.tags.first().map(String::clone).unwrap_or_default(),
                    tags: x.meta.tags.clone(),
                    pubDate: format_time(&x.meta.date, format),
                })
                .collect::<Vec<_>>(),
        },
    };

    let (template, filename) = match format {
        FeedFormat::Atom => ("atom", "atom.xml"),
        FeedFormat::RSS => ("rss", "rss.xml"),
    };

    let contents = handlebars.render(template, &rss)?;
    let file_path = config.outdir.join(folder).join(filename);
    println!("writing {} to {}...", template, file_path.display());
    let mut file = std::fs::File::create(file_path).context("generate_feed")?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

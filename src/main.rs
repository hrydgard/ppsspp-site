// Things to generate:
// - [...] Docs
// - [ ] Pages
//   - [ ] Downloads
//   - [ ] Regular static pages
// - [ ] Blog
// - [ ] News
//
// Features to add:
// - templating
// - choose css framework
// - serve

use std::{
    io::Write,
    path::{Path, PathBuf},
};

extern crate anyhow;
mod config;
mod document;
mod util;
use anyhow::Context;
pub use config::Config;

extern crate serde;
use serde::{Deserialize, Serialize};

use document::*;

fn generate_docnav_html(root: &document::Category, focused_doc_path: &Path) -> String {
    let mut str = String::new();
    str
}

fn generate_doctree(config: &Config, folder: &str) -> anyhow::Result<()> {
    // First, build the tree and convert all the markdown to html and metadata.
    let root_folder = config.indir.join(folder);
    let out_root_folder = config.outdir.clone();
    let root_cat = document::Category::from_folder_tree(&root_folder, &config.markdown_options)?;

    // Write out all the files. Don't need recursion here so we can linearize.
    let docs = root_cat.all_documents();

    for doc in docs {
        let mut target_path = out_root_folder.join(doc.path);
        target_path.set_extension("");

        util::create_folder_if_missing(&target_path)?;

        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(doc.html.as_bytes())?;
    }

    // MD documents get wrapped into our doc templat&e.
    // println!("{:#?}", root_cat);

    Ok(())
}

fn generate_blog(config: &Config, folder: &str) -> anyhow::Result<()> {
    // For the blog

    let root = config.indir.join(folder);
    let out_root = config.outdir.join(folder);

    Ok(())
}

fn generate_pages(config: &Config, folder: &str) -> anyhow::Result<()> {
    let root_folder = config.indir.join(folder);
    // pages are generated directly into the root.
    let out_root_folder = &config.outdir;

    let listing = root_folder.read_dir()?;
    for entry in listing {
        let entry = entry?;
        let path = root_folder.join(entry.file_name());
        let mut file_name = PathBuf::from(entry.file_name());
        // TODO: Parse metadata out of the files somehow!
        let Some(os_str) = path.extension() else {
            continue;
        };
        println!("considering {}", path.display());
        let document = match os_str.to_str().unwrap() {
            "md" => {
                file_name.set_extension("html");
                Document::from_md(&path, &config.markdown_options)?
            }
            "html" => Document::from_html(&path)?,
            "js" => {
                continue;
            }
            _ => {
                println!("Ignoring {}", path.display());
                continue;
            }
        };
        let target_path = out_root_folder.join(file_name);
        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(document.html.as_bytes())?;
    }
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();

    //handlebars.register_template_file("header", "template/common_header.hbs")?;
    //handlebars.register_template_file("footer", "template/common_footer.hbs")?;

    println!("Barebones website generator");

    let config = Config {
        indir: PathBuf::from("."),
        outdir: PathBuf::from("build"),
        markdown_options: markdown::Options::gfm(),
    };

    if !config.outdir.exists() {
        std::fs::create_dir(&config.outdir).context("outdir")?;
    }

    generate_pages(&config, "src/pages")?;

    generate_blog(&config, "blog")?;
    generate_blog(&config, "news")?;

    generate_doctree(&config, "docs")?;
    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}

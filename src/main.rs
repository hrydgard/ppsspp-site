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

use document::*;

fn generate_docnav_html(root: &document::Category, focused_doc_path: &Path) -> String {
    let mut str = String::new();
    str
}

fn generate_doctree(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<()> {
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

        // We apply the template right here.
        let context = PageContext::new(Some(doc.meta.title), Some(doc.html));
        let html = handlebars.render("doc", &context)?;

        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(html.as_bytes())?;
    }

    // MD documents get wrapped into our doc template.
    // println!("{:#?}", root_cat);

    Ok(())
}

fn generate_blog(config: &Config, folder: &str) -> anyhow::Result<()> {
    // For the blog

    let root = config.indir.join(folder);
    let out_root = config.outdir.join(folder);

    Ok(())
}

fn generate_pages(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<()> {
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
        let (document, apply_doc_template) = match os_str.to_str().unwrap() {
            "md" => {
                file_name.set_extension("html");
                (Document::from_md(&path, &config.markdown_options)?, true)
            }
            "html" => (Document::from_html(&path)?, true),
            "hbs" => {
                file_name.set_extension("html");
                (Document::from_hbs(&path, handlebars)?, false)
            }
            "js" => {
                continue;
            }
            _ => {
                println!("Ignoring {}", path.display());
                continue;
            }
        };

        let html = if apply_doc_template {
            let context = PageContext::new(Some(document.meta.title), Some(document.html));
            handlebars.render("doc", &context)?
        } else {
            document.html
        };

        let target_path = out_root_folder.join(file_name);
        println!("Writing {}", target_path.display());
        let mut file = std::fs::File::create(&target_path).context("create_file")?;
        file.write_all(html.as_bytes())?;
    }
    Ok(())
}

fn run() -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_template_file("common_header", "template/common_header.hbs")?;
    handlebars.register_template_file("common_footer", "template/common_footer.hbs")?;
    handlebars.register_template_file("doc", "template/doc.hbs");

    println!("Barebones website generator");

    let mut markdown_options = markdown::Options::gfm();
    println!("md: {:#?}", markdown_options);

    markdown_options.compile.allow_dangerous_html = true;

    let config = Config {
        indir: PathBuf::from("."),
        outdir: PathBuf::from("build"),
        markdown_options,
    };

    if !config.outdir.exists() {
        std::fs::create_dir(&config.outdir).context("outdir")?;
    }

    println!("Copying static files...");

    util::copy_recursive(config.indir.join("static"), config.outdir.join("static"))?;
    std::fs::copy(
        config.indir.join("static/img/favicon.ico"),
        config.outdir.join("favicon.ico"),
    )?;

    generate_pages(&config, "src/pages", &mut handlebars)?;

    generate_doctree(&config, "docs", &mut handlebars)?;

    generate_blog(&config, "blog")?;
    generate_blog(&config, "news")?;

    Ok(())
}

fn main() {
    env_logger::init();

    run().unwrap();
}

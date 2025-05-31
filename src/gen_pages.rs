use crate::document::{Document, PageContext};
use crate::{config::*, util};
use anyhow::Context;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_pages(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<Vec<Document>> {
    println!("Generating pages from {folder}...");

    let root_folder = config.in_dir.join(folder);
    anyhow::ensure!(root_folder.exists());
    // pages are generated directly into the root.
    let out_root_folder = &config.out_dir;

    let listing = root_folder.read_dir()?;
    for entry in listing {
        let entry = entry?;
        let path = root_folder.join(entry.file_name());
        let mut file_name = PathBuf::from(entry.file_name());
        // TODO: Parse metadata out of the files somehow!
        let Some(os_str) = path.extension() else {
            continue;
        };
        let name = util::strip_extension(entry.file_name());
        let (document, apply_doc_template) = match os_str.to_str().unwrap() {
            "md" => {
                file_name.set_extension("html");
                (Document::from_md(&path, config)?, true)
            }
            "html" => (Document::from_html(&path)?, true),
            "hbs" => {
                file_name.set_extension("html");
                (
                    Document::from_hbs(&config, &name, &path, handlebars)?,
                    false,
                )
            }
            "js" => {
                continue;
            }
            _ => {
                println!("Ignoring {}", path.display());
                continue;
            }
        };

        let target_path = out_root_folder.join(file_name);
        let fname = util::filename_to_string(&entry.file_name());

        let html = if apply_doc_template {
            let mut context = PageContext::from_document(&document, &config);
            context.globals = Some(&config.global_meta);
            if let Some(ref mut meta) = &mut context.meta {
                meta.url = format!(
                    "/{}",
                    fname.as_str().strip_suffix(".hbs").unwrap_or_default()
                );
            }
            context.render("page", handlebars)?
        } else {
            document.html
        };

        if fname == "index.hbs" {
            println!("index.html special case");
            // Just write it plain.
            let mut file = std::fs::File::create(&target_path).context("create_file_as_dir")?;
            file.write_all(html.as_bytes())?;
        } else {
            // Otherwise, get rid of the extension by putting it in a subdirectory.
            util::write_file_as_folder_with_index(&target_path, html, true)?;
        }
    }
    println!("Wrote pages from {}", folder);
    Ok(vec![])
}

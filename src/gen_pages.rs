use crate::document::{Document, PageContext};
use crate::{config::*, util};
use std::path::PathBuf;

pub fn generate_pages(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<Vec<Document>> {
    println!("Generating pages from '{folder}'...");

    let root_folder = config.in_dir.join(folder);
    anyhow::ensure!(root_folder.exists());
    // pages are generated directly into the root.
    let out_root_folder = &config.out_dir;

    let mut documents = vec![];
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
        let (mut document, apply_doc_template) = match os_str.to_str().unwrap() {
            "md" => {
                file_name.set_extension("html");
                (Document::from_md(&path, config)?, true)
            }
            "html" => (Document::from_html(&path)?, true),
            "hbs" => {
                file_name.set_extension("html");
                (
                    Document::from_hbs(&config.global_meta, &name, &path, handlebars)?,
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
        document.meta.url = format!("/{name}");

        let html = if apply_doc_template {
            let mut context = PageContext::from_document(&document, &config.global_meta);
            context.globals = Some(&config.global_meta);
            context.render("page", handlebars)?
        } else {
            std::mem::take(&mut document.html)
        };

        if fname == "index.hbs" {
            println!("index.html special case");
            // Just write it plain.
            util::write_html(&target_path, &html)?;
        } else {
            // Otherwise, get rid of the extension by putting it in a subdirectory.
            util::write_file_as_folder_with_index(&target_path, html, true)?;
        }
        documents.push(document);
    }
    println!("Wrote pages from '{}'", folder);
    // Deterministic order (read_dir isn't), for the sitemap.
    documents.sort_by(|a, b| a.meta.url.cmp(&b.meta.url));
    Ok(documents)
}

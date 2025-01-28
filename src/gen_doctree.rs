use crate::document::{self, Category, Document, PageContext};
use crate::index;
use crate::{config::*, util};
use anyhow::Context;
use std::io::Write;

// TODO: Involve templates here for easier modification?
// Can handlebars templates recurse?
// Should be surrounded in an <ul class="nav-tree">
fn generate_docnav_html(root: &Category, level: usize, breadcrumbs: &[DocLink]) -> String {
    let mut str = String::new();

    str += &format!("<ul class=\"nav-tree-items level-{}\">\n", level);
    for cat in &root.sub_categories {
        let expanded = if let Some(crumb) = breadcrumbs.get(level + 1) {
            cat.meta.url == crumb.url
        } else {
            false
        };

        let extra_classes = if expanded { " selected" } else { "" };
        str += &format!(
            "<li><a href=\"{}\" class=\"nav-tree-category{}\">{}</a>",
            cat.meta.url, extra_classes, cat.meta.title
        );
        if expanded {
            str += &generate_docnav_html(cat, level + 1, breadcrumbs);
        }
        str += "</li>\n";
    }
    for doc in &root.documents {
        let expanded = if let Some(crumb) = breadcrumbs.get(level + 1) {
            doc.meta.url == crumb.url
        } else {
            false
        };
        let extra_classes = if expanded { " selected" } else { "" };
        str += &format!(
            "<li><a href=\"{}\" class=\"nav-tree-item {}\">{}</a></li>\n",
            doc.meta.url, extra_classes, doc.meta.title,
        );
    }
    str += "</ul>\n";

    str
}

pub fn generate_doctree(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<Vec<Document>> {
    println!("Generating doctree from {folder}...");
    // First, build the tree and convert all the markdown to html and metadata.
    let root_folder = config.in_dir.join(folder);
    anyhow::ensure!(root_folder.exists());
    let out_root_folder = config.out_dir.clone();
    let mut root_cat = document::Category::from_folder_tree(&root_folder, config).context("from_folder_tree")?;

    let mut crumbs = vec![DocLink {
        title: "Docs".to_owned(),
        url: format!("/{}", folder),
        date: "N/A".to_owned(),
        summary: None,
        external: false,
        selected: false,
        position: 0,
    }];
    root_cat.compute_breadcrumbs(&mut crumbs);

    // Write out all the docs. Don't need recursion here so we can linearize.
    // Note that we also generate the categories as documents in `all_documents`.
    let mut docs = root_cat.all_documents(handlebars, &config.global_meta)?;
    Category::add_prev_next_links(&mut docs);

    for doc in &docs {
        let target_path = out_root_folder.join(&doc.path);

        util::create_folder_if_missing(&target_path)?;

        // We apply the template right here.
        let mut context = PageContext::from_document(doc, &config.global_meta);
        context.sidebar = Some(generate_docnav_html(&root_cat, 0, &doc.meta.breadcrumbs));
        let html = context.render("doc", handlebars).context("render")?;

        util::write_file_as_folder_with_index(&target_path, html, true)?;
    }

    let mut index = index::Index::new();

    // Generate search index. Could be done in parallel to writing out the files.
    for doc in &docs {
        if let Some(markdown) = &doc.markdown {
            index.add_md(&config.markdown_options, markdown, &doc.meta).context("add_md")?;
        }
    }

    let json_index = index.to_index_json();
    let json_index_path = out_root_folder.join("index.json");
    let mut file = std::fs::File::create(&json_index_path).context("create_json_index")?;
    file.write_all(json_index.as_bytes())?;

    println!(
        "Wrote doctree {}, index as {}",
        folder,
        json_index_path.display()
    );

    Ok(docs)
}

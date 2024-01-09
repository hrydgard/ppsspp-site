// Things to generate:
// - [x] Docs
// - [x] Pages
//   - [x] Downloads
//   - [x] Regular static pages
// - [x] Blog
// - [x] News
// Features to add:
// - [x] serve, with proxy
// - [x] templating
// - [x] choose css framework (none)
// - [x] Javascript basics (log in, log out)
// - [x] Purchase flow
// - [x] Admin UI
// - [x] generate RSS/Atom feeds
// - [x] summaries
// - [x] Pretty alerts
// - [x] Replace inline SVG
// - [x] Docs tree view
// - [x] Screenshot gallery
// - [x] Light/dark mode
// - [ ] Styling forms
// - [x] Mobile site improvements (move login to popdown menu, fix scrolling)
// - [ ] Polish
// - [ ] Test purchase
// - [ ] *** Deploy ***
// - [ ] h2 Section links
// - [ ] Blog tags, browse by
// - [ ] Nicer author tags in the blog
// - [ ] Merge the two blogs
// - [ ] Blog feed pagination
// - [ ] Docs search

use chrono::{DateTime, Utc};
use std::{
    cmp::Ordering,
    convert::TryInto,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};

extern crate anyhow;
mod config;
mod document;
mod feed;
mod index;
mod post_process;
mod server;
mod util;

use anyhow::Context;
pub use config::Config;
use notify::Watcher;
use structopt::StructOpt;

extern crate serde;

use document::*;

use crate::{
    config::{DocLink, GlobalMeta},
    util::{filename_to_string, write_file_as_folder_with_index},
};

fn generate_doctree(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<()> {
    // First, build the tree and convert all the markdown to html and metadata.
    let root_folder = config.indir.join(folder);
    anyhow::ensure!(root_folder.exists());
    let out_root_folder = config.outdir.clone();
    let mut root_cat = document::Category::from_folder_tree(&root_folder, config)?;

    let mut crumbs = vec![DocLink {
        title: "Docs".to_owned(),
        url: format!("/{}", folder),
        summary: None,
        external: false,
        selected: false,
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
        let html = context.render("doc", handlebars)?;

        write_file_as_folder_with_index(&target_path, html, true)?;
    }

    let mut index = index::Index::new();

    // Generate search index. Could be done in parallel to writing out the files.
    for doc in docs {
        if let Some(markdown) = &doc.markdown {
            index.add_md(&config.markdown_options, markdown, &doc.meta)?;
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

    Ok(())
}

// Posts should be passed-in in reverse time order.
fn generate_blog_sidebar(
    title: &str,
    url: &str,
    all_posts: &[Document],
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<String> {
    let context = SidebarContext {
        title: title.to_string(),
        links: all_posts
            .iter()
            .map(|doc| doc.to_doclink(url))
            .collect::<Vec<_>>(),
    };

    let output = handlebars.render("blog_sidebar", &context)?;
    Ok(output)
}

fn generate_blog(
    config: &Config,
    folder: &str,
    title: &str,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<Vec<Document>> {
    // For the blog

    let root_folder = config.indir.join(folder);
    anyhow::ensure!(root_folder.exists());
    let out_root_folder = config.outdir.join(folder);

    util::create_folder_if_missing(&out_root_folder)?;

    let mut documents = vec![];

    let listing = root_folder.read_dir()?;

    let mut tag_lookup = std::collections::HashMap::<String, Tag>::new();

    for entry in listing {
        let entry = entry?;
        let file_name = PathBuf::from(entry.file_name());
        let Some(os_str) = file_name.extension() else {
            continue;
        };
        match os_str.to_str().unwrap() {
            "md" => {}
            _ => {
                println!("Skipping file {}", file_name.display());
                continue;
            }
        }
        let name = util::filename_to_string(&entry.file_name());

        let parts: [&str; 4] = name.splitn(4, '-').collect::<Vec<_>>().try_into().unwrap();

        let mut doc = Document::from_md(&root_folder.join(entry.file_name()), config)?;

        let [year, month, day, remainder] = parts;
        doc.meta.date = format!("{}-{}-{}", year, month, day);
        if doc.meta.slug.is_empty() {
            println!(
                "Warning: Blog entry missing slug, autodetecting {}: {}",
                name, remainder
            );
            doc.meta.slug = remainder.to_string();
        }
        assert!(!doc.meta.slug.is_empty());
        doc.meta.url = format!("/{folder}/{}", &doc.meta.slug);
        doc.path = out_root_folder.join(&doc.meta.slug);

        for tag in &doc.meta.tags {
            tag_lookup
                .entry(tag.clone())
                .or_insert_with(|| Tag {
                    name: tag.clone(),
                    articles: vec![],
                })
                .articles
                .push(doc.to_doclink(""));
        }

        documents.push(doc);
    }

    documents.sort_by(|a, b| {
        a.meta
            .date
            .partial_cmp(&b.meta.date)
            .unwrap_or(Ordering::Equal)
            .reverse()
    });

    // Reformat the tag data to a vector.
    let tags = tag_lookup.values().cloned().collect::<Vec<_>>();

    // Add next/forward links
    // for [prev, cur, next] in documents.
    for i in 0..documents.len() {
        if let Some(prev) = documents.get(i.wrapping_sub(1)) {
            documents[i].meta.prev = Some(prev.to_doclink(""));
        }
        if let Some(next) = documents.get(i.wrapping_add(1)) {
            documents[i].meta.next = Some(next.to_doclink(""));
        }
    }

    for doc in &documents {
        let context = PageContext::from_document(doc, &config.global_meta);

        // First, render the blog post itself, without the surrounding chrome. This is so that we can add on
        // more blog posts underneath later for a more continuous experience.
        let post_html = handlebars.render("blog_post", &context)?;
        let sidebar = generate_blog_sidebar(title, &doc.meta.url, &documents, handlebars)?;

        let mut context = PageContext::from_document(doc, &config.global_meta);
        // Now, use that as contents and render into a doc template.
        context.contents = Some(post_html);
        context.sidebar = Some(sidebar);
        //println!("{:#?}", context.meta);
        let html = context.render("doc", handlebars)?;

        let target_path = &doc.path;
        util::write_file_as_folder_with_index(target_path, html, false)?;
    }

    // Generate RSS feed
    feed::write_feed(
        config,
        title,
        "PPSSPP Blog",
        folder,
        &documents,
        feed::FeedFormat::Atom,
        handlebars,
    )?;
    feed::write_feed(
        config,
        title,
        "PPSSPP Blog",
        folder,
        &documents,
        feed::FeedFormat::RSS,
        handlebars,
    )?;

    // Generate the root blog post.
    let sidebar = generate_blog_sidebar(title, &format!("/{}", folder), &documents, handlebars)?;
    // First, render the blog post itself, without the surrounding chrome. This is so that we can add on
    // more blog posts underneath later for a more continuous experience.
    let post_html = documents
        .iter()
        .map(|doc| {
            let context = PageContext::from_document(doc, &config.global_meta);
            // Now, use that as contents and render into a doc template.
            context.render("blog_post", handlebars).unwrap()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mut context =
        PageContext::new(Some(title.to_owned()), Some(post_html), &config.global_meta);
    context.sidebar = Some(sidebar);
    context.tags = tags;
    context.meta = Some(documents[0].meta.clone());

    let html = context.render("doc", handlebars)?;

    let target_path = out_root_folder;
    util::write_file_as_folder_with_index(&target_path, html, false)?;

    println!("Wrote blog {}", folder);

    Ok(documents)
}

fn generate_pages(
    config: &Config,
    folder: &str,
    handlebars: &mut handlebars::Handlebars,
) -> anyhow::Result<()> {
    let root_folder = config.indir.join(folder);
    anyhow::ensure!(root_folder.exists());
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
        let name = strip_extension(entry.file_name());
        let (document, apply_doc_template) = match os_str.to_str().unwrap() {
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
        let fname = filename_to_string(&entry.file_name());

        let html = if apply_doc_template {
            let mut context = PageContext::from_document(&document, &config.global_meta);
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
    Ok(())
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, default_value = "3000")]
    port: i32,
    #[structopt(long)]
    prod: bool,
    #[structopt(long)]
    minify: bool,
    #[structopt(long)]
    skip_serve: bool,
}

fn build(opt: &Opt) -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();

    handlebars.register_template_file("common_header", "template/common_header.hbs")?;
    handlebars.register_template_file("common_footer", "template/common_footer.hbs")?;
    handlebars.register_template_file("doc", "template/doc.hbs")?;
    handlebars.register_template_file("link_icon", "template/icons/link.hbs")?;
    handlebars.register_template_file("cat_contents", "template/cat_contents.hbs")?;
    handlebars.register_template_file("blog_post", "template/blog_post.hbs")?;
    handlebars.register_template_file("blog_sidebar", "template/blog_sidebar.hbs")?;
    handlebars.register_template_file("product_card", "template/product_card.hbs")?;
    handlebars.register_template_file("page", "template/page.hbs")?;
    handlebars.register_template_file("rss", "template/rss.hbs")?;
    handlebars.register_template_file("atom", "template/atom.hbs")?;

    println!("PPSSPP website generator");

    let mut markdown_options = markdown::Options::gfm();
    markdown_options.compile.allow_dangerous_html = true;
    // println!("md: {:#?}", markdown_options);

    let url_base = if opt.prod {
        "https://www.ppsspp.org"
    } else {
        "https://dev.ppsspp.org"
    }
    .to_string();

    let top_nav: Vec<DocLink> =
        serde_json::from_str(&std::fs::read_to_string("data/top_nav.json")?)?;

    let current_time: DateTime<Utc> = Utc::now();

    // Format the time in the desired format
    let formatted_time = current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    println!("Build time: {formatted_time}");

    let config = Config {
        url_base: url_base.clone(),
        indir: PathBuf::from("."),
        outdir: PathBuf::from("build"),
        markdown_options,
        global_meta: GlobalMeta::new(opt.prod, &url_base, top_nav)?,
        build_date: formatted_time,
        github_url: "https://github.com/hrydgard/ppsspp/issues/",
    };

    if !config.outdir.exists() {
        std::fs::create_dir(&config.outdir).context("outdir")?;
    }

    util::copy_recursive(
        config.indir.join("static"),
        config.outdir.join("static"),
        opt.minify,
        &["css"], // We mash the css files together, so don't copy them.
    )?;
    // Move the favicon into place.
    std::fs::copy(
        config.indir.join("static/img/favicon.ico"),
        config.outdir.join("favicon.ico"),
    )?;
    // Concat the CSS files.
    util::concat_files(
        &config.indir.join("static/css"),
        &[
            "vars.css",
            "reset.css",
            "grid.css",
            "style.css",
            "top-nav.css",
            "ui.css",
            "hamburger.css",
            "gallery.css",
            "hero.css",
            "highlight-dark.min.css",
        ],
        &config.outdir.join("static/css/all.css"),
        !opt.minify,
    )?;

    generate_doctree(&config, "docs", &mut handlebars)?;

    let _ = generate_blog(&config, "blog", "Development blog", &mut handlebars)?;
    let _ = generate_blog(&config, "news", "Release News", &mut handlebars)?;

    generate_pages(&config, "pages", &mut handlebars)?;

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let opt = Opt::from_args();

    build(&opt).unwrap();

    if opt.skip_serve {
        println!("not serving.");
        return Ok(());
    }

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| match res {
            Ok(event) => {
                if matches!(event.kind, notify::EventKind::Modify(_)) {
                    notify_tx.send(()).unwrap();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let watch_dirs = &[
        "blog", "data", "docs", "news", "pages", "static", "template",
    ];
    for dir in watch_dirs {
        watcher.watch(Path::new(dir), notify::RecursiveMode::Recursive)?;
    }

    // OK, we're done - just serve the results.
    println!("Serving on localhost:{}", opt.port);

    server::spawn_server(opt.port as u16).await;

    let mut quit = false;
    while !quit {
        // Only look for changes every second, to kinda batch them up.
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let mut changed = false;
        loop {
            let result = notify_rx.try_recv();
            match result {
                Ok(()) => {
                    // Received something.
                    changed = true;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    quit = true;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    break;
                }
            }
        }

        if changed {
            // TODO: Could make it more fine grained, but for now we just rebuild everything,
            // it's fast enough.
            println!("Detected changes, rebuilding!");
            build(&opt).unwrap();
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}

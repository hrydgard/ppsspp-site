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
// - [x] Styling forms
// - [x] Mobile site improvements (move login to popdown menu, fix scrolling)
// - [x] Polish
// - [x] Test purchase
// - [x] *** Deploy ***
// - [ ] h2 Section links
// - [ ] Blog tags, browse by
// - [ ] Nicer author tags in the blog
// - [ ] Merge the two blogs
// - [ ] Blog feed pagination
// - [ ] Docs search
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![allow(clippy::too_many_arguments)]

use chrono::{Datelike, DateTime, Utc};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

mod config;
mod document;
mod feed;
mod gen_blog;
mod gen_doctree;
mod gen_pages;
mod gen_sitemap;
mod index;
mod post_process;
mod server;
mod util;

use anyhow::Context;
use clap::Parser;
pub use config::Config;
use notify::Watcher;

use crate::config::{DocLink, GlobalMeta};

#[allow(dead_code)]
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value_t = 3000)]
    port: i32,
    #[arg(long)]
    prod: bool,
    #[arg(long)]
    dev: bool,
    #[arg(long)]
    minify: bool,
    #[arg(long)]
    skip_serve: bool,
}

fn build(opt: &Args) -> anyhow::Result<()> {
    let mut handlebars = handlebars::Handlebars::new();

    let templates = &[
        "common_header",
        "common_footer",
        "doc",
        "cat_contents",
        "blog_post",
        "blog_page",
        "blog_sidebar",
        "unit",
        "product_card",
        "page",
        "feed_rss",
        "feed_atom",
        "sitemap_xml",
    ];
    for tmpl in templates {
        handlebars.register_template_file(tmpl, &format!("template/{tmpl}.hbs")).context("register_template")?;
    }
    handlebars.register_template_file("link_icon", "template/icons/link_icon.hbs").context("register_last_template")?;

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
        serde_json::from_str(&std::fs::read_to_string("data/top_nav.json")?).context("json")?;

    let current_time: DateTime<Utc> = Utc::now();

    // Format the time in the desired format
    let formatted_time = current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    println!("Build time: {formatted_time}");

    let mut config = Config {
        url_base: url_base.clone(),
        in_dir: PathBuf::from("."),
        out_dir: PathBuf::from("build"),
        markdown_options,
        global_meta: GlobalMeta::new(opt.prod, &url_base, top_nav)?,
        build_date: formatted_time,
        build_year: current_time.year(),
        github_url: "https://github.com/hrydgard/ppsspp/issues/",
    };

    if !config.out_dir.exists() {
        std::fs::create_dir(&config.out_dir).context("out dir")?;
    }

    util::copy_recursive(
        config.in_dir.join("static"),
        config.out_dir.join("static"),
        opt.minify,
        &["css"], // We mash the css files together, so don't copy them.
    )?;
    // Move the favicon into place.
    std::fs::copy(
        config.in_dir.join("static/img/favicon.ico"),
        config.out_dir.join("favicon.ico"),
    )?;
    // Concat the CSS files.
    util::concat_files(
        &config.in_dir.join("static/css"),
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
            "icons.css",
        ],
        &config.out_dir.join("static/css/all.css"),
        !opt.minify,
    )?;

    let docs = gen_doctree::generate_doctree(&config, "docs", &mut handlebars)?;

    let blog = gen_blog::generate_blog(&config, "blog", "Development blog", &mut handlebars)?;
    let news = gen_blog::generate_blog(&config, "news", "Release News", &mut handlebars)?;

    config.global_meta.latest_news = news
        .iter()
        .take(3)
        .map(|doc| doc.to_doclink(""))
        .collect::<Vec<_>>();

    let pages = gen_pages::generate_pages(&config, "pages", &mut handlebars)?;

    let mut sitemap = gen_sitemap::SitemapGenerator::new();
    sitemap.add(&docs, 0.8);
    sitemap.add(&blog, 0.9);
    sitemap.add(&news, 0.9);
    sitemap.add(&pages, 1.0);
    sitemap.generate(&config, &mut handlebars)?;

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let opt = Args::parse();

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

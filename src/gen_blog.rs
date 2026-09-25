use crate::{config::*, document::*, feed, util};
use serde::Serialize;
use std::path::{Path, PathBuf};

// Number of posts per listing page (/news, /news/page/2, ...). Tag listings are paginated the same way.
pub const POSTS_PER_PAGE: usize = 5;

// Number of most recent posts listed in the sidebar. Older ones are reachable through the archive page.
const SIDEBAR_RECENT_POSTS: usize = 20;

// Put one of these on a line of its own (with blank lines around it) in a post to show only
// the part above it in listings, followed by a "Read more" link.
const EXCERPT_MARKERS: &[&str] = &["<!-- more -->", "<!--more-->"];

fn excerpt(html: &str) -> Option<&str> {
    EXCERPT_MARKERS
        .iter()
        .filter_map(|marker| html.find(marker))
        .min()
        .map(|pos| &html[..pos])
}

// Posts should be passed-in in reverse time order.
fn generate_blog_sidebar(
    title: &str,
    url: &str,
    root_url: &str,
    all_posts: &[&Document],
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<String> {
    let mut links = all_posts
        .iter()
        .take(SIDEBAR_RECENT_POSTS)
        .map(|doc| doc.to_doclink(url))
        .collect::<Vec<_>>();
    // Keep the current post visible even if it's older than the recent ones.
    if let Some(doc) = all_posts
        .iter()
        .skip(SIDEBAR_RECENT_POSTS)
        .find(|doc| doc.meta.url == url)
    {
        links.push(doc.to_doclink(url));
    }

    let context = SidebarContext {
        title: title.to_string(),
        root_url: root_url.to_string(),
        archive_url: format!("{root_url}/archive"),
        links,
    };

    let output = handlebars.render("blog_sidebar", &context)?;
    Ok(output)
}

pub fn generate_blog(
    config: &Config,
    folder: &str,
    title: &str,
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<Vec<Document>> {
    println!("Generating blog from '{folder}'...");

    // For the blog

    let root_folder = config.in_dir.join(folder);
    anyhow::ensure!(root_folder.exists());
    let out_root_folder = config.out_dir.join(folder);

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
                println!("Skipping file '{}'", file_name.display());
                continue;
            }
        }
        let name = util::filename_to_string(&entry.file_name());

        let parts: [&str; 4] = name.splitn(4, '-').collect::<Vec<_>>().try_into().unwrap();

        let mut doc = Document::from_md(&root_folder.join(entry.file_name()), config)?;

        let [year, month, day, remainder] = parts;
        doc.meta.section = folder.to_string();
        doc.meta.date = format!("{}-{}-{}", year, month, day);
        if doc.meta.slug.is_empty() {
            println!(
                "Warning: Blog entry '{}' missing slug, auto-detecting: '{}'",
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
                    selected: false,
                })
                .articles
                .push(doc.to_doclink(""));
        }

        documents.push(doc);
    }

    // Newest first. Posts from the same day are ordered by slug, so the output doesn't depend on
    // directory listing order.
    documents.sort_by(|a, b| {
        b.meta
            .date
            .cmp(&a.meta.date)
            .then_with(|| a.meta.slug.cmp(&b.meta.slug))
    });

    // Reformat the tag data to a vector.
    let mut tags = tag_lookup.values().cloned().collect::<Vec<_>>();
    tags.sort_by_key(|t| t.name.clone());

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

    let all_posts = documents.iter().collect::<Vec<_>>();

    for doc in &documents {
        let context = PageContext::from_document(doc, &config.global_meta);

        // First, render the blog post itself, without the surrounding chrome. This is so that we can add on
        // more blog posts underneath later for a more continuous experience.
        let post_html = handlebars.render("blog_post", &context)?;
        let sidebar = generate_blog_sidebar(title, &doc.meta.url, &format!("/{}", folder), &all_posts, handlebars)?;

        let mut context = PageContext::from_document(doc, &config.global_meta);
        // Now, use that as contents and render into a doc template.
        context.contents = Some(post_html);
        context.sidebar = Some(sidebar);
        //println!("{:#?}", context.meta);
        let html = context.render("blog_page", handlebars)?;

        let target_path = &doc.path;
        util::write_file_as_folder_with_index(target_path, html, false)?;
    }

    // Generate RSS feed
    let feed_title = format!("PPSSPP {title}");
    feed::write_feed(
        config,
        &feed_title,
        "PPSSPP news, release notes and development articles",
        folder,
        &documents,
        feed::FeedFormat::Atom,
        handlebars,
    )?;
    feed::write_feed(
        config,
        &feed_title,
        "PPSSPP news, release notes and development articles",
        folder,
        &documents,
        feed::FeedFormat::RSS,
        handlebars,
    )?;

    // Generate the paginated listing of all posts.
    generate_listing(
        config,
        &all_posts,
        folder,
        title,
        title,
        &format!("/{folder}"),
        &out_root_folder,
        &tags,
        handlebars,
    )?;

    // Now for each tag, generate another, but filtered by tag.
    for tag in &tags {
        // Mark this tag as selected for rendering
        let mut tags_with_selection = tags.clone();
        for t in &mut tags_with_selection {
            t.selected = t.name == tag.name;
        }
        let tagged_posts = documents
            .iter()
            .filter(|doc| doc.meta.tags.contains(&tag.name))
            .collect::<Vec<_>>();
        generate_listing(
            config,
            &tagged_posts,
            folder,
            title,
            &format!("{title}: {}", tag.name),
            &format!("/{folder}/tags/{}", tag.name),
            &out_root_folder.join("tags").join(&tag.name),
            &tags_with_selection,
            handlebars,
        )?;
    }

    generate_archive(config, &all_posts, folder, title, &out_root_folder, handlebars)?;

    println!("Wrote blog '{}'", folder);

    Ok(documents)
}

fn page_url(base_url: &str, page: usize) -> String {
    if page == 1 {
        base_url.to_string()
    } else {
        format!("{base_url}/page/{page}")
    }
}

// First, last, and the pages around the current one, with gaps in between.
fn page_links(base_url: &str, current: usize, total: usize) -> Vec<PageLink> {
    let mut links = vec![];
    let mut last_shown = 0;
    for page in 1..=total {
        if page != 1 && page != total && page.abs_diff(current) > 2 {
            continue;
        }
        if page > last_shown + 1 {
            links.push(PageLink {
                number: None,
                url: String::new(),
                current: false,
            });
        }
        links.push(PageLink {
            number: Some(page),
            url: page_url(base_url, page),
            current: page == current,
        });
        last_shown = page;
    }
    links
}

// Writes a paginated listing of the posts to out_path (page 1) and out_path/page/N.
// Posts should be passed-in in reverse time order.
fn generate_listing(
    config: &Config,
    posts: &[&Document],
    folder: &str,
    title: &str,
    page_title: &str,
    base_url: &str,
    out_path: &Path,
    all_tags: &[Tag],
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<()> {
    let sidebar = generate_blog_sidebar(title, base_url, &format!("/{folder}"), posts, handlebars)?;

    let total = posts.len().div_ceil(POSTS_PER_PAGE);
    for (i, chunk) in posts.chunks(POSTS_PER_PAGE).enumerate() {
        let page = i + 1;

        let post_html = chunk
            .iter()
            .map(|doc| {
                let mut context = PageContext::from_document(doc, &config.global_meta);
                context.is_list_view = true;
                if let Some(excerpt) = excerpt(&doc.html) {
                    context.contents = Some(excerpt.to_string());
                    context.truncated = true;
                }
                context.render("blog_post", handlebars)
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .join("\n");

        let page_title = if page == 1 {
            page_title.to_string()
        } else {
            format!("{page_title} (page {page})")
        };
        let mut context =
            PageContext::new(Some(page_title), Some(post_html), &config.global_meta);
        context.sidebar = Some(sidebar.clone());
        context.tags = all_tags;
        context.meta = Some(DocumentMeta {
            url: page_url(base_url, page),
            section: folder.to_string(),
            ..Default::default()
        });
        if total > 1 {
            context.pagination = Some(Pagination {
                current: page,
                total,
                newer_url: (page > 1).then(|| page_url(base_url, page - 1)),
                older_url: (page < total).then(|| page_url(base_url, page + 1)),
                pages: page_links(base_url, page, total),
            });
        }

        let html = context.render("blog_page", handlebars)?;

        let target_path = if page == 1 {
            out_path.to_path_buf()
        } else {
            out_path.join("page").join(page.to_string())
        };
        util::write_file_as_folder_with_index(&target_path, html, false)?;
    }
    Ok(())
}

#[derive(Serialize)]
struct ArchiveYear {
    year: String,
    links: Vec<DocLink>,
}

#[derive(Serialize)]
struct ArchiveContext {
    title: String,
    years: Vec<ArchiveYear>,
}

// A single page listing the titles of all posts, grouped by year.
// Posts should be passed-in in reverse time order.
fn generate_archive(
    config: &Config,
    posts: &[&Document],
    folder: &str,
    title: &str,
    out_root_folder: &Path,
    handlebars: &mut handlebars::Handlebars<'_>,
) -> anyhow::Result<()> {
    let mut years: Vec<ArchiveYear> = vec![];
    for doc in posts {
        let year = doc.meta.date.split('-').next().unwrap_or_default();
        if years.last().is_none_or(|y| y.year != year) {
            years.push(ArchiveYear {
                year: year.to_string(),
                links: vec![],
            });
        }
        years.last_mut().unwrap().links.push(doc.to_doclink(""));
    }

    let archive_title = format!("{title}: all posts");
    let contents = handlebars.render(
        "blog_archive",
        &ArchiveContext {
            title: archive_title.clone(),
            years,
        },
    )?;

    let url = format!("/{folder}/archive");
    let mut context = PageContext::new(Some(archive_title), Some(contents), &config.global_meta);
    context.sidebar = Some(generate_blog_sidebar(title, &url, &format!("/{folder}"), posts, handlebars)?);
    context.meta = Some(DocumentMeta {
        url,
        section: folder.to_string(),
        ..Default::default()
    });

    let html = context.render("blog_page", handlebars)?;
    util::write_file_as_folder_with_index(&out_root_folder.join("archive"), html, false)?;
    Ok(())
}

use crate::config::Config;
use crate::document::*;
use anyhow::Context;
use markdown::mdast::Node;

#[allow(clippy::single_match)]
fn recurse_text_join(nodes: &[Node], str: &mut String) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                *str += &text.value;
            }
            Node::InlineCode(code) => {
                *str += &code.value;
            }
            _ => {}
        }
        if let Some(children) = node.children() {
            recurse_text_join(children, str);
        }
    }
}

// Cuts at a char boundary (byte slicing could panic on non-ASCII text).
fn truncate_string(str: String, max_length: usize) -> String {
    if str.len() <= max_length {
        str
    } else {
        let mut end = max_length;
        while !str.is_char_boundary(end) {
            end -= 1;
        }
        // TODO: cut at whole words.
        format!("{}...", &str[0..end])
    }
}

// Collapses whitespace and cuts at a word boundary, for <meta name="description">.
pub fn make_description(text: &str) -> String {
    const MAX_DESCRIPTION_CHARS: usize = 155;
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() <= MAX_DESCRIPTION_CHARS {
        return text;
    }
    let cut: String = text.chars().take(MAX_DESCRIPTION_CHARS).collect();
    let cut = match cut.rfind(' ') {
        Some(pos) if pos > MAX_DESCRIPTION_CHARS / 2 => &cut[..pos],
        _ => cut.as_str(),
    };
    format!("{}…", cut.trim_end_matches(|c: char| c.is_ascii_punctuation() || c == ' '))
}

#[allow(clippy::single_match)]
fn markdown_summary_recurse(nodes: &[Node], meta: &mut DocumentMeta) -> anyhow::Result<()> {
    const MAX_SUMMARY_CHARS: usize = 200;

    if !meta.title.is_empty() && meta.summary.is_some() {
        return Ok(());
    }
    for node in nodes {
        match node {
            Node::Heading(heading) => {
                match heading.children.first().context("missing heading child")? {
                    Node::Text(text) => {
                        if meta.title.is_empty() {
                            meta.title = text.value.clone();
                        }
                    }
                    _ => {}
                }
            }
            Node::Paragraph(_) => {
                if meta.summary.is_none() {
                    let summary = paragraph_text(node);
                    // Skip paragraphs without text, like a lone image.
                    if !summary.trim().is_empty() {
                        meta.summary = Some(truncate_string(summary, MAX_SUMMARY_CHARS));
                    }
                }
            }
            _ => {
                if let Some(children) = node.children() {
                    markdown_summary_recurse(children, meta)?;
                }
            }
        }
    }
    Ok(())
}

pub fn add_meta_from_markdown(
    markdown: &str,
    meta: &mut DocumentMeta,
    config: &Config,
) -> anyhow::Result<()> {
    // If no dash-meta, grab the title string.
    let tree = markdown::to_mdast(markdown, &config.markdown_options.parse).unwrap();
    markdown_summary_recurse(std::slice::from_ref(&tree), meta)?;
    if meta.description.is_none() {
        meta.description = description_from_paragraphs(&tree);
    }
    Ok(())
}

fn paragraph_text(node: &Node) -> String {
    let mut text = String::new();
    if let Some(children) = node.children() {
        recurse_text_join(children, &mut text);
    }
    text
}

// For <meta name="description"> without `description:` front matter: the text of the first
// top-level paragraphs, enough of them to say something ("1.20.4 is out!" alone doesn't).
// Paragraphs without text (a lone image) are skipped.
fn description_from_paragraphs(tree: &Node) -> Option<String> {
    const MIN_DESCRIPTION_CHARS: usize = 110;
    const MAX_PARAGRAPHS: usize = 3;
    let mut text = String::new();
    let paragraphs = tree
        .children()?
        .iter()
        .filter(|node| matches!(node, Node::Paragraph(_)))
        .map(paragraph_text)
        .filter(|t| !t.trim().is_empty())
        .take(MAX_PARAGRAPHS);
    for paragraph in paragraphs {
        // A paragraph ending in ':' introduces a list or code block, which isn't included,
        // so stop before it (unless it's the first).
        let paragraph = paragraph.trim();
        if !text.is_empty() {
            if paragraph.ends_with(':') {
                break;
            }
            text.push(' ');
        }
        text += paragraph;
        if text.ends_with(':') || text.chars().count() >= MIN_DESCRIPTION_CHARS {
            break;
        }
    }
    (!text.is_empty()).then(|| make_description(&text))
}

// Markdown pre-processing. This is for linking GitHub issues.
pub fn preprocess_markdown(md: &str, doc_name: &str, config: &Config) -> anyhow::Result<String> {
    anyhow::ensure!(
        md.contains("github") || !md.contains(".md)"),
        "Markdown {} contains bad link",
        doc_name
    );

    // Relative links break: every page is served as a folder (/docs/a/b/), so "c" or "docs/c"
    // resolves to /docs/a/b/c. Links must be absolute (/docs/...), external, mailto: or #anchors.
    let link_regex = regex::Regex::new(r"\]\(([^)\s]+)").unwrap();
    for captures in link_regex.captures_iter(md) {
        let target = &captures[1];
        anyhow::ensure!(
            ["/", "#", "http://", "https://", "mailto:"]
                .iter()
                .any(|prefix| target.starts_with(prefix)),
            "Markdown {} has a relative link '{}', use an absolute path like /docs/... instead",
            doc_name,
            target
        );
        // "/docs//x" is served as a duplicate page under another URL.
        anyhow::ensure!(
            !(target.starts_with('/') && target.contains("//")),
            "Markdown {} has a link with a double slash: '{}'",
            doc_name,
            target
        );
    }

    // The optional "(" catches references that are already links, which are left alone.
    let issue_regex = regex::Regex::new(r"\[#(\d+)\](\()?").unwrap();
    Ok(issue_regex
        .replace_all(md, |captures: &regex::Captures<'_>| {
            if captures.get(2).is_some() {
                return captures[0].to_string();
            }
            let issue_number = captures.get(1).unwrap().as_str();
            format!("[#{}]({}{})", issue_number, config.github_url, issue_number)
            // Construct the replacement with the GitHub URL
        })
        .to_string())
}

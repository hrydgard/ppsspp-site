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
            _ => {}
        }
        if let Some(children) = node.children() {
            recurse_text_join(children, str);
        }
    }
}

fn truncate_string(str: String, max_length: usize) -> String {
    if str.len() <= max_length {
        str
    } else {
        // TODO: cut at whole words.
        format!("{}...", &str[0..max_length])
    }
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
                    let mut summary = String::new();
                    if let Some(children) = node.children() {
                        recurse_text_join(children, &mut summary);
                    }
                    meta.summary = Some(truncate_string(summary, MAX_SUMMARY_CHARS));
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
    markdown_summary_recurse(&[tree], meta)?;
    Ok(())
}

// Markdown post-processing. This is for linking GitHub issues.
pub fn preprocess_markdown(md: &str, doc_name: &str, config: &Config) -> anyhow::Result<String> {
    anyhow::ensure!(
        md.contains("github") || !md.contains(".md)"),
        "Markdown {} contains bad link",
        doc_name
    );

    let issue_regex = regex::Regex::new(r"\[#(\d+)\]").unwrap();
    Ok(issue_regex
        .replace_all(md, |captures: &regex::Captures<'_>| {
            let issue_number = captures.get(1).unwrap().as_str();
            format!("[#{}]({}{})", issue_number, config.github_url, issue_number)
            // Construct the replacement with the GitHub URL
        })
        .to_string())
}

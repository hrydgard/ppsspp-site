// Some ideas:
// https://javascript.plainenglish.io/building-a-simple-in-browser-search-engine-d87c86ac3261

use std::collections::HashMap;

use crate::document::DocumentMeta;
use markdown::mdast::Node;
use serde::Serialize;

#[derive(Serialize)]
pub struct Index {
    doc_meta: Vec<DocumentMeta>,
    index: HashMap<String, Vec<u32>>,
}

const PUNCTUATION: &[char] = &[',', '.', '!', ':', '?', '(', ')'];

fn recurse_markdown_tree(nodes: &[Node], words: &mut Vec<String>) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                words.extend(
                    text.value
                        .split_ascii_whitespace()
                        .map(|s| s.trim_matches(PUNCTUATION).to_string().to_lowercase()),
                );
            }
            _ => {
                if let Some(children) = node.children() {
                    recurse_markdown_tree(children, words);
                }
            }
        }
    }
}

fn split_markdown_to_words(
    options: &markdown::Options,
    markdown: &str,
) -> anyhow::Result<Vec<String>> {
    let tree = markdown::to_mdast(markdown, &options.parse).unwrap();

    let mut words: Vec<String> = vec![];
    recurse_markdown_tree(&[tree], &mut words);
    Ok(words)
}

impl Index {
    pub fn new() -> Self {
        Self {
            doc_meta: vec![],
            index: Default::default(),
        }
    }

    pub fn add_md(
        &mut self,
        options: &markdown::Options,
        markdown: &str,
        meta: &DocumentMeta,
    ) -> anyhow::Result<()> {
        self.doc_meta.push(meta.clone());
        let doc_index = (self.doc_meta.len() - 1) as u32;

        let words = split_markdown_to_words(options, markdown)?;
        for word in words {
            self.index.entry(word).or_default().push(doc_index);
        }

        Ok(())
    }

    // Save the index for javascript use
    pub fn to_index_json(&self) -> String {
        serde_json::to_string(&self.index).unwrap()
    }

    // Note: We don't actually have a method here for performing searches, that'll be all client side.
}

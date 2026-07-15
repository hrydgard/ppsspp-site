use std::collections::HashMap;

use markdown::mdast;

pub struct RenderParams {
    pub min_toc_headers: usize,
    pub max_toc_depth: u8,
    pub h1_title_above_toc: bool,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            min_toc_headers: 3,
            max_toc_depth: 4,
            h1_title_above_toc: true,
        }
    }
}

#[derive(Clone)]
struct DefinitionData {
    url: String,
    title: Option<String>,
}

#[derive(Clone)]
struct HeadingData {
    depth: u8,
    id: String,
    title: String,
}

struct TocData {
    headings: Vec<HeadingData>,
    min_depth: u8,
}

pub fn to_html_with_options(
    markdown: &str,
    options: &markdown::Options,
    params: &RenderParams,
) -> Result<String, markdown::message::Message> {
    let tree = markdown::to_mdast(markdown, &options.parse)?;
    let mut renderer = Renderer::new(options, params);
    renderer.collect_definitions(&tree);
    renderer.render_root(&tree);
    Ok(renderer.finish())
}

struct Renderer<'a> {
    options: &'a markdown::Options,
    params: &'a RenderParams,
    out: String,
    definitions: HashMap<String, DefinitionData>,
    footnote_definitions: HashMap<String, mdast::FootnoteDefinition>,
    footnote_order: Vec<String>,
    footnote_ref_counts: HashMap<String, usize>,
    headings: Vec<HeadingData>,
    heading_slug_counts: HashMap<String, usize>,
    next_heading_index: usize,
    promoted_h1_index: Option<usize>,
}

impl<'a> Renderer<'a> {
    fn new(options: &'a markdown::Options, params: &'a RenderParams) -> Self {
        Self {
            options,
            params,
            out: String::new(),
            definitions: HashMap::new(),
            footnote_definitions: HashMap::new(),
            footnote_order: Vec::new(),
            footnote_ref_counts: HashMap::new(),
            headings: Vec::new(),
            heading_slug_counts: HashMap::new(),
            next_heading_index: 0,
            promoted_h1_index: None,
        }
    }

    fn finish(mut self) -> String {
        self.render_footnotes();
        self.out
    }

    fn collect_definitions(&mut self, node: &mdast::Node) {
        match node {
            mdast::Node::Definition(def) => {
                self.definitions.insert(
                    def.identifier.clone(),
                    DefinitionData {
                        url: def.url.clone(),
                        title: def.title.clone(),
                    },
                );
            }
            mdast::Node::FootnoteDefinition(def) => {
                self.footnote_definitions
                    .insert(def.identifier.clone(), def.clone());
            }
            mdast::Node::Heading(heading) => {
                let mut title = plain_text_from_nodes(&heading.children);
                title = title.trim().to_string();

                let base = if title.is_empty() {
                    "section".to_string()
                } else {
                    let slug = slugify_identifier(&title);
                    if slug.is_empty() {
                        "section".to_string()
                    } else {
                        slug
                    }
                };

                let count = self
                    .heading_slug_counts
                    .entry(base.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                let id = if *count == 1 {
                    base
                } else {
                    format!("{}-{}", base, count)
                };

                self.headings.push(HeadingData {
                    depth: heading.depth,
                    id,
                    title,
                });

                for child in &heading.children {
                    self.collect_definitions(child);
                }
            }
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        self.collect_definitions(child);
                    }
                }
            }
        }
    }

    fn render_root(&mut self, node: &mdast::Node) {
        if let mdast::Node::Root(root) = node {
            let toc_data = self.collect_toc_data();

            if let Some(toc_data) = toc_data {
                if self.params.h1_title_above_toc {
                    if let Some((idx, heading)) = self
                        .headings
                        .iter()
                        .enumerate()
                        .find(|(_, heading)| heading.depth == 1)
                    {
                        self.promoted_h1_index = Some(idx);
                        self.out.push_str("<h1 id=\"");
                        self.out.push_str(&escape_html_attr(&heading.id));
                        self.out.push_str("\">");
                        self.out.push_str(&escape_html_text(&heading.title));
                        self.out.push_str("</h1>");
                    }
                }

                self.render_toc(&toc_data);
            }

            for child in &root.children {
                self.render_flow(child, false);
            }
        }
    }

    fn render_flow(&mut self, node: &mdast::Node, tight: bool) {
        match node {
            mdast::Node::Root(root) => {
                for child in &root.children {
                    self.render_flow(child, false);
                }
            }
            mdast::Node::Paragraph(paragraph) => {
                if tight {
                    self.render_inlines(&paragraph.children);
                } else {
                    self.out.push_str("<p>");
                    self.render_inlines(&paragraph.children);
                    self.out.push_str("</p>");
                }
            }
            mdast::Node::Heading(heading) => {
                let heading_index = self.next_heading_index;
                self.next_heading_index += 1;

                if self.promoted_h1_index == Some(heading_index) {
                    return;
                }

                let heading_id = self
                    .headings
                    .get(heading_index)
                    .map(|h| h.id.clone())
                    .unwrap_or_else(|| format!("h-{}", heading_index + 1));

                self.out.push_str(&format!("<h{} id=\"", heading.depth));
                self.out.push_str(&escape_html_attr(&heading_id));
                self.out.push_str("\">");
                self.render_inlines(&heading.children);
                self.out.push_str(&format!("</h{}>", heading.depth));
            }
            mdast::Node::ThematicBreak(_) => self.out.push_str("<hr />"),
            mdast::Node::Blockquote(blockquote) => {
                self.out.push_str("<blockquote>");
                for child in &blockquote.children {
                    self.render_flow(child, false);
                }
                self.out.push_str("</blockquote>");
            }
            mdast::Node::List(list) => self.render_list(list),
            mdast::Node::Code(code) => {
                self.out.push_str("<pre><code");
                if let Some(lang) = &code.lang {
                    self.out.push_str(" class=\"language-");
                    self.out.push_str(&escape_html_attr(lang));
                    self.out.push('"');
                }
                self.out.push('>');
                self.out.push_str(&escape_html_text(&code.value));
                if !code.value.ends_with('\n') {
                    self.out.push('\n');
                }
                self.out.push_str("</code></pre>");
            }
            mdast::Node::Math(math) => {
                self.out
                    .push_str("<pre><code class=\"language-math math-display\">");
                self.out.push_str(&escape_html_text(&math.value));
                if !math.value.ends_with('\n') {
                    self.out.push('\n');
                }
                self.out.push_str("</code></pre>");
            }
            mdast::Node::Html(html) => {
                if self.options.compile.allow_dangerous_html {
                    self.out.push_str(&html.value);
                } else {
                    self.out.push_str(&escape_html_text(&html.value));
                }
            }
            mdast::Node::Table(table) => self.render_table(table),
            mdast::Node::Definition(_) | mdast::Node::FootnoteDefinition(_) => {}
            mdast::Node::MdxjsEsm(esm) => {
                self.out.push_str(&escape_html_text(&esm.value));
            }
            mdast::Node::Toml(_) | mdast::Node::Yaml(_) => {}
            mdast::Node::MdxFlowExpression(expr) => {
                self.out.push_str(&escape_html_text(&expr.value));
            }
            mdast::Node::ListItem(item) => {
                self.render_list_item(item, false);
            }
            _ => {
                if let Some(children) = node.children() {
                    for child in children {
                        self.render_flow(child, false);
                    }
                }
            }
        }
    }

    fn render_inlines(&mut self, nodes: &[mdast::Node]) {
        for node in nodes {
            self.render_inline(node);
        }
    }

    fn render_inline(&mut self, node: &mdast::Node) {
        match node {
            mdast::Node::Text(text) => self.out.push_str(&escape_html_text(&text.value)),
            mdast::Node::InlineCode(code) => {
                self.out.push_str("<code>");
                self.out.push_str(&escape_html_text(&code.value));
                self.out.push_str("</code>");
            }
            mdast::Node::InlineMath(math) => {
                self.out
                    .push_str("<code class=\"language-math math-inline\">");
                self.out.push_str(&escape_html_text(&math.value));
                self.out.push_str("</code>");
            }
            mdast::Node::Break(_) => self.out.push_str("<br />\n"),
            mdast::Node::Emphasis(emphasis) => {
                self.out.push_str("<em>");
                self.render_inlines(&emphasis.children);
                self.out.push_str("</em>");
            }
            mdast::Node::Strong(strong) => {
                self.out.push_str("<strong>");
                self.render_inlines(&strong.children);
                self.out.push_str("</strong>");
            }
            mdast::Node::Delete(delete) => {
                self.out.push_str("<del>");
                self.render_inlines(&delete.children);
                self.out.push_str("</del>");
            }
            mdast::Node::Link(link) => {
                self.out.push_str("<a href=\"");
                self.out
                    .push_str(&safe_url(&link.url, false, &self.options.compile));
                self.out.push('"');
                if let Some(title) = &link.title {
                    self.out.push_str(" title=\"");
                    self.out.push_str(&escape_html_attr(title));
                    self.out.push('"');
                }
                self.out.push('>');
                self.render_inlines(&link.children);
                self.out.push_str("</a>");
            }
            mdast::Node::LinkReference(link_ref) => {
                let def = self.definitions.get(&link_ref.identifier).cloned();
                let (url, title) = if let Some(def) = def {
                    (def.url, def.title)
                } else {
                    (String::new(), None)
                };

                self.out.push_str("<a href=\"");
                self.out
                    .push_str(&safe_url(&url, false, &self.options.compile));
                self.out.push('"');
                if let Some(title) = title {
                    self.out.push_str(" title=\"");
                    self.out.push_str(&escape_html_attr(&title));
                    self.out.push('"');
                }
                self.out.push('>');
                self.render_inlines(&link_ref.children);
                self.out.push_str("</a>");
            }
            mdast::Node::Image(image) => {
                self.render_image(&image.url, image.title.as_deref(), &image.alt);
            }
            mdast::Node::ImageReference(image_ref) => {
                let def = self.definitions.get(&image_ref.identifier).cloned();
                if let Some(def) = def {
                    self.render_image(&def.url, def.title.as_deref(), &image_ref.alt);
                } else {
                    self.render_image("", None, &image_ref.alt);
                }
            }
            mdast::Node::FootnoteReference(reference) => {
                self.render_footnote_reference(reference);
            }
            mdast::Node::Html(html) => {
                if self.options.compile.allow_dangerous_html {
                    self.out.push_str(&html.value);
                } else {
                    self.out.push_str(&escape_html_text(&html.value));
                }
            }
            mdast::Node::MdxTextExpression(expr) => {
                self.out.push_str(&escape_html_text(&expr.value));
            }
            mdast::Node::MdxJsxTextElement(jsx) => {
                self.render_inlines(&jsx.children);
            }
            _ => {
                if let Some(children) = node.children() {
                    self.render_inlines(children);
                }
            }
        }
    }

    fn render_list(&mut self, list: &mdast::List) {
        let tight = !list.spread
            && list
                .children
                .iter()
                .all(|n| matches!(n, mdast::Node::ListItem(li) if !li.spread));

        if list.ordered {
            self.out.push_str("<ol");
            if let Some(start) = list.start {
                if start != 1 {
                    self.out.push_str(" start=\"");
                    self.out.push_str(&start.to_string());
                    self.out.push('"');
                }
            }
            self.out.push('>');
        } else {
            self.out.push_str("<ul>");
        }

        for child in &list.children {
            if let mdast::Node::ListItem(item) = child {
                self.render_list_item(item, tight);
            }
        }

        if list.ordered {
            self.out.push_str("</ol>");
        } else {
            self.out.push_str("</ul>");
        }
    }

    fn render_list_item(&mut self, item: &mdast::ListItem, tight: bool) {
        self.out.push_str("<li>");

        if let Some(checked) = item.checked {
            self.out.push_str("<input type=\"checkbox\"");
            if checked {
                self.out.push_str(" checked=\"\"");
            }
            if !self.options.compile.gfm_task_list_item_checkable {
                self.out.push_str(" disabled=\"\"");
            }
            self.out.push_str(" /> ");
        }

        for child in &item.children {
            match child {
                mdast::Node::Paragraph(paragraph) if tight => {
                    self.render_inlines(&paragraph.children);
                }
                _ => self.render_flow(child, false),
            }
        }

        self.out.push_str("</li>");
    }

    fn render_table(&mut self, table: &mdast::Table) {
        self.out.push_str("<table>");

        let mut rows = table.children.iter();
        if let Some(mdast::Node::TableRow(head_row)) = rows.next() {
            self.out.push_str("<thead><tr>");
            for (index, cell_node) in head_row.children.iter().enumerate() {
                if let mdast::Node::TableCell(cell) = cell_node {
                    self.out.push_str("<th");
                    if let Some(align) = table.align.get(index) {
                        self.push_align_attr(*align);
                    }
                    self.out.push('>');
                    self.render_inlines(&cell.children);
                    self.out.push_str("</th>");
                }
            }
            self.out.push_str("</tr></thead>");
        }

        self.out.push_str("<tbody>");
        for row_node in rows {
            if let mdast::Node::TableRow(row) = row_node {
                self.out.push_str("<tr>");
                for (index, cell_node) in row.children.iter().enumerate() {
                    if let mdast::Node::TableCell(cell) = cell_node {
                        self.out.push_str("<td");
                        if let Some(align) = table.align.get(index) {
                            self.push_align_attr(*align);
                        }
                        self.out.push('>');
                        self.render_inlines(&cell.children);
                        self.out.push_str("</td>");
                    }
                }
                self.out.push_str("</tr>");
            }
        }
        self.out.push_str("</tbody></table>");
    }

    fn push_align_attr(&mut self, align: mdast::AlignKind) {
        match align {
            mdast::AlignKind::Left => self.out.push_str(" align=\"left\""),
            mdast::AlignKind::Right => self.out.push_str(" align=\"right\""),
            mdast::AlignKind::Center => self.out.push_str(" align=\"center\""),
            mdast::AlignKind::None => {}
        }
    }

    fn render_image(&mut self, url: &str, title: Option<&str>, alt: &str) {
        self.out.push_str("<img src=\"");
        self.out
            .push_str(&safe_url(url, true, &self.options.compile));
        self.out.push_str("\" alt=\"");
        self.out.push_str(&escape_html_attr(alt));
        self.out.push('"');
        if let Some(title) = title {
            self.out.push_str(" title=\"");
            self.out.push_str(&escape_html_attr(title));
            self.out.push('"');
        }
        self.out.push_str(" />");
    }

    fn render_footnote_reference(&mut self, reference: &mdast::FootnoteReference) {
        let count = self
            .footnote_ref_counts
            .entry(reference.identifier.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        if !self.footnote_order.contains(&reference.identifier) {
            self.footnote_order.push(reference.identifier.clone());
        }

        let index = self
            .footnote_order
            .iter()
            .position(|id| id == &reference.identifier)
            .map(|n| n + 1)
            .unwrap_or(1);

        let prefix = footnote_clobber_prefix(&self.options.compile);
        let safe_ident = slugify_identifier(&reference.identifier);
        let ref_id = if *count == 1 {
            format!("{prefix}fnref-{safe_ident}")
        } else {
            format!("{prefix}fnref-{safe_ident}-{count}")
        };
        let note_id = format!("{prefix}fn-{safe_ident}");

        self.out.push_str("<sup><a href=\"#");
        self.out.push_str(&escape_html_attr(&note_id));
        self.out.push_str("\" id=\"");
        self.out.push_str(&escape_html_attr(&ref_id));
        self.out
            .push_str("\" data-footnote-ref=\"\" aria-describedby=\"footnote-label\">");
        self.out.push_str(&index.to_string());
        self.out.push_str("</a></sup>");
    }

    fn render_footnotes(&mut self) {
        if self.footnote_order.is_empty() {
            return;
        }

        let label_tag = self
            .options
            .compile
            .gfm_footnote_label_tag_name
            .as_deref()
            .unwrap_or("h2");
        let label_attrs = self
            .options
            .compile
            .gfm_footnote_label_attributes
            .as_deref()
            .unwrap_or("class=\"sr-only\"");
        let label_text = self
            .options
            .compile
            .gfm_footnote_label
            .as_deref()
            .unwrap_or("Footnotes");
        let back_label = self
            .options
            .compile
            .gfm_footnote_back_label
            .as_deref()
            .unwrap_or("Back to content");
        let prefix = footnote_clobber_prefix(&self.options.compile);

        self.out
            .push_str("<section data-footnotes=\"\" class=\"footnotes\">");
        self.out.push('<');
        self.out.push_str(label_tag);
        self.out.push_str(" id=\"footnote-label\"");
        if !label_attrs.is_empty() {
            self.out.push(' ');
            self.out.push_str(label_attrs);
        }
        self.out.push('>');
        self.out.push_str(&escape_html_text(label_text));
        self.out.push_str("</");
        self.out.push_str(label_tag);
        self.out.push_str("><ol>");

        let footnote_order = self.footnote_order.clone();
        for identifier in &footnote_order {
            let note_id = format!("{prefix}fn-{}", slugify_identifier(identifier));
            self.out.push_str("<li id=\"");
            self.out.push_str(&escape_html_attr(&note_id));
            self.out.push_str("\">");

            if let Some(definition) = self.footnote_definitions.get(identifier).cloned() {
                if definition.children.is_empty() {
                    self.out.push_str("<p>");
                }

                for child in &definition.children {
                    self.render_flow(child, false);
                }

                if definition.children.is_empty() {
                    self.out.push_str("</p>");
                }
            } else {
                self.out.push_str("<p></p>");
            }

            let refs = self.footnote_ref_counts.get(identifier).copied().unwrap_or(1);
            for ref_index in 1..=refs {
                let backref_id = if ref_index == 1 {
                    format!("{prefix}fnref-{}", slugify_identifier(identifier))
                } else {
                    format!("{prefix}fnref-{}-{ref_index}", slugify_identifier(identifier))
                };

                self.out.push_str(" <a href=\"#");
                self.out.push_str(&escape_html_attr(&backref_id));
                self.out.push_str("\" data-footnote-backref=\"\" aria-label=\"");
                self.out.push_str(&escape_html_attr(back_label));
                self.out.push_str("\" class=\"data-footnote-backref\">↩");
                if ref_index > 1 {
                    self.out.push_str("<sup>");
                    self.out.push_str(&ref_index.to_string());
                    self.out.push_str("</sup>");
                }
                self.out.push_str("</a>");
            }

            self.out.push_str("</li>");
        }

        self.out.push_str("</ol></section>");
    }

    fn collect_toc_data(&self) -> Option<TocData> {
        let max_depth = self.params.max_toc_depth.clamp(2, 6);
        let toc_headings: Vec<HeadingData> = self
            .headings
            .iter()
            .filter(|h| (2..=max_depth).contains(&h.depth))
            .cloned()
            .collect();

        if toc_headings.len() < self.params.min_toc_headers {
            return None;
        }

        let min_depth = toc_headings.iter().map(|h| h.depth).min().unwrap_or(2);
        Some(TocData {
            headings: toc_headings,
            min_depth,
        })
    }

    fn render_toc(&mut self, toc_data: &TocData) {
        let toc_headings = &toc_data.headings;
        let min_depth = toc_data.min_depth;

        self.out
            .push_str("<nav class=\"toc\" aria-label=\"Table of contents\"><ul>");

        let mut prev_depth: u8 = min_depth;
        let mut first = true;

        for heading in toc_headings {
            let mut depth = heading.depth;
            if !first && depth > prev_depth + 1 {
                depth = prev_depth + 1;
            }

            if first {
                first = false;
            } else if depth > prev_depth {
                for _ in prev_depth..depth {
                    self.out.push_str("<ul>");
                }
            } else if depth < prev_depth {
                for _ in depth..prev_depth {
                    self.out.push_str("</li></ul>");
                }
                self.out.push_str("</li>");
            } else {
                self.out.push_str("</li>");
            }

            self.out.push_str("<li><a href=\"#");
            self.out.push_str(&escape_html_attr(&heading.id));
            self.out.push_str("\">");
            self.out.push_str(&escape_html_text(&heading.title));
            self.out.push_str("</a>");

            prev_depth = depth;
        }

        self.out.push_str("</li>");
        while prev_depth > min_depth {
            self.out.push_str("</ul></li>");
            prev_depth -= 1;
        }
        self.out.push_str("</ul></nav>");
    }
}

fn footnote_clobber_prefix(options: &markdown::CompileOptions) -> &str {
    options
        .gfm_footnote_clobber_prefix
        .as_deref()
        .unwrap_or("user-content-")
}

fn escape_html_text(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            _ => out.push(ch),
        }
    }
    out
}

fn escape_html_attr(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

fn safe_url(url: &str, image: bool, options: &markdown::CompileOptions) -> String {
    let sanitized = markdown::sanitize(url);

    if options.allow_dangerous_protocol {
        return sanitized;
    }

    if image && options.allow_any_img_src {
        return sanitized;
    }

    let protocols: &[&str] = if image {
        &["http", "https"]
    } else {
        &["http", "https", "irc", "ircs", "mailto", "xmpp"]
    };

    let end = sanitized.find(|c| matches!(c, '?' | '#' | '/'));
    let mut colon = sanitized.find(':');

    if let Some(end) = end {
        if let Some(index) = colon {
            if index > end {
                colon = None;
            }
        }
    }

    if let Some(colon) = colon {
        let protocol = sanitized[0..colon].to_lowercase();
        if !protocols.contains(&protocol.as_str()) {
            return String::new();
        }
    }

    sanitized
}

fn slugify_identifier(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn plain_text_from_nodes(nodes: &[mdast::Node]) -> String {
    let mut out = String::new();
    for node in nodes {
        match node {
            mdast::Node::Text(text) => out.push_str(&text.value),
            mdast::Node::InlineCode(code) => out.push_str(&code.value),
            mdast::Node::InlineMath(math) => out.push_str(&math.value),
            mdast::Node::Image(image) => out.push_str(&image.alt),
            mdast::Node::ImageReference(image_ref) => out.push_str(&image_ref.alt),
            _ => {
                if let Some(children) = node.children() {
                    out.push_str(&plain_text_from_nodes(children));
                }
            }
        }
    }
    out
}
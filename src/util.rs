use anyhow::Context;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde_json::Value;
use std::sync::LazyLock;

// Short content hash of a file, for ?v= cache busting. FNV-1a, so it's stable across
// Rust versions and builds (std's DefaultHasher isn't guaranteed to be).
pub fn file_version(path: &Path) -> anyhow::Result<String> {
    let bytes = fs::read(path).with_context(|| format!("file_version {}", path.display()))?;
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    Ok(format!("{hash:016x}")[..10].to_string())
}

pub const SITE_URL: &str = "https://www.ppsspp.org";

// The canonical form of a site path: with a trailing slash, since every page is a folder
// with an index.html and nginx 301-redirects the slashless form. "/index" is the front page.
pub fn canonical_path(url: &str) -> String {
    let url = url.trim_end_matches('/');
    if url.is_empty() || url == "/index" {
        "/".to_string()
    } else {
        format!("{url}/")
    }
}

// Rewrites internal links like href="/download" or href="/news/foo#bar" to the slashed
// form, to save visitors (and crawlers) a redirect. Leaves alone files, protocol-relative
// and external URLs, and /api/.
// "/static/x.mp4" and "/news/atom.xml" are files, "/news/release-1.20.4" is a page:
// a file extension is alphanumeric, contains a letter, and isn't too long ("webmanifest" is 11).
fn is_file_path(path: &str) -> bool {
    let last_segment = path.rsplit('/').next().unwrap_or_default();
    match last_segment.rsplit_once('.') {
        Some((_, ext)) => {
            (1..=12).contains(&ext.len())
                && ext.chars().all(|c| c.is_ascii_alphanumeric())
                && ext.chars().any(|c| c.is_ascii_alphabetic())
        }
        None => false,
    }
}

pub fn add_trailing_slashes(html: &str) -> String {
    static HREF: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r#"href="(/[^"?#]*)([?#][^"]*)?""#).unwrap());
    HREF.replace_all(html, |caps: &regex::Captures<'_>| {
        let path = &caps[1];
        let rest = caps.get(2).map_or("", |m| m.as_str());
        if path.ends_with('/') || path.starts_with("//") || path.starts_with("/api/") || is_file_path(path) {
            caps[0].to_string()
        } else {
            format!("href=\"{path}/{rest}\"")
        }
    })
    .into_owned()
}

pub fn strip_extension(str: OsString) -> String {
    let mut x = PathBuf::from(str);
    x.set_extension("");
    x.to_string_lossy().to_string()
}

pub fn filename_to_string(name: &OsStr) -> String {
    // name.to_str().unwrap().to_owned()
    name.to_string_lossy().to_string()
}

#[allow(clippy::single_match)]
pub fn copy_recursive(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    minify: bool,
    exclude_dirs: &[&str],
) -> anyhow::Result<()> {
    let minify_session = minify_js::Session::new();
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            let name = filename_to_string(&entry.file_name());
            if !exclude_dirs.contains(&name.as_str()) {
                copy_recursive(
                    entry.path(),
                    dst.as_ref().join(entry.file_name()),
                    minify,
                    exclude_dirs,
                )
                .context("copy-recurse")?;
            } else {
                // Just create the empty dir.
                fs::create_dir_all(&dst.as_ref().join(entry.file_name()))?;
            }
        } else {
            let dst = dst.as_ref().join(entry.file_name());
            if minify {
                // TODO: This complains about utf-8. Oh well, let's fix this later.

                // TODO: This read can be done more efficiently.
                let mut data = std::fs::read_to_string(entry.path())?.as_bytes().to_owned();

                // Here we can minify.
                if let Some(os_str) = entry.path().extension() {
                    // Check file extension to figure out what to do.
                    match os_str.to_str().unwrap() {
                        "js" => {
                            let mut buffer = vec![];
                            let _ = minify_js::minify(
                                &minify_session,
                                minify_js::TopLevelMode::Global,
                                &data,
                                &mut buffer,
                            );
                            data = buffer;
                        }
                        _ => {}
                    }
                }

                let mut out_file = std::fs::File::create(&dst).context("minify")?;
                out_file.write_all(&data)?;
            }

            fs::copy(entry.path(), dst).context("copy-file")?;
        }
    }
    Ok(())
}

pub fn write_file_as_folder_with_index(
    path: &Path,
    contents: String,
    strip_extension: bool,
) -> anyhow::Result<()> {
    let mut extensionless = path.to_path_buf();
    if strip_extension {
        extensionless.set_extension("");
    }

    let _ = std::fs::create_dir_all(&extensionless);
    let file_path = extensionless.join("index.html");
    write_html(&file_path, &contents)
}

// All generated HTML pages should be written through this.
pub fn write_html(file_path: &Path, contents: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(file_path).context("write_html")?;
    file.write_all(add_trailing_slashes(contents).as_bytes())?;
    Ok(())
}

pub fn create_folder_if_missing(path: &Path) -> anyhow::Result<()> {
    // Create folder if missing.
    let parent = path.parent().unwrap();
    if !parent.exists() {
        println!("creating {}", parent.display());
        std::fs::create_dir_all(parent).context("create_dir_if_missing")
    } else {
        Ok(())
    }
}

pub fn concat_files(
    in_parent: &Path,
    inputs: &[&str],
    out_path: &Path,
    include_headings: bool,
) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(out_path)?;
    for input in inputs {
        if include_headings {
            let heading = format!("\n\n/* ============== {} ============== */\n\n", input);
            file.write_all(heading.as_bytes())?;
        }
        let data = std::fs::read(in_parent.join(input))?;
        file.write_all(&data)?;
    }
    Ok(())
}

/// Your global, immutable column mapping.
/// Order here = Order in the Markdown table.
const COLUMN_MAPPING: &[(&str, &str)] = &[
    ("Server Name", "name"),
    ("Host", "host"),
    ("Location", "location"),
    ("Community", "discord"),
];

// TODO: This needs refinement.
pub fn table_from_adhoc_servers(json_str: &str) -> anyhow::Result<String> {
    json_to_markdown_table(json_str, "servers", COLUMN_MAPPING)
        .map_err(|e| anyhow::anyhow!("Failed to create markdown table: {}", e))
}

pub fn json_to_markdown_table(
    json_str: &str,
    array_name: &str,
    mapping: &[(&str, &str)]
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let v: Value = serde_json::from_str(json_str)?;

    let servers = v[array_name].as_array()
        .ok_or(format!("JSON must contain a '{}' array", array_name))?;

    let mut table = String::new();

    // 1. Headers
    table.push_str("| ");
    table.push_str(&mapping.iter().map(|(title, _)| *title).collect::<Vec<_>>().join(" | "));
    table.push_str(" |\n");

    // 2. Separator
    table.push_str("| ");
    table.push_str(&mapping.iter().map(|_| "---").collect::<Vec<_>>().join(" | "));
    table.push_str(" |\n");

    // 3. Data Rows
    for server in servers {
        let mut row = Vec::new();
        for (_, key) in mapping {
            // Fetch value, handle non-string types gracefully, escape pipes
            let val = server.get(*key)
                .map(|v| v.as_str().unwrap_or("").replace('|', "\\|"))
                .unwrap_or_default();
            row.push(val);
        }
        table.push_str("| ");
        table.push_str(&row.join(" | "));
        table.push_str(" |\n");
    }

    Ok(table)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailing_slashes() {
        let cases = [
            (r#"<a href="/download">"#, r#"<a href="/download/">"#),
            (r#"<a href="/">"#, r#"<a href="/">"#),
            (r#"<a href="/news/release-1.20">"#, r#"<a href="/news/release-1.20/">"#),
            (r#"<a href="/news/release-1.20.4#x">"#, r#"<a href="/news/release-1.20.4/#x">"#),
            (r#"<a href="/news/coming-soon-0.9-3">"#, r#"<a href="/news/coming-soon-0.9-3/">"#),
            (r#"<a href="/login?Forward=download">"#, r#"<a href="/login/?Forward=download">"#),
            (r#"<link rel="manifest" href="/manifest.webmanifest">"#, r#"<link rel="manifest" href="/manifest.webmanifest">"#),
            (r#"<a href="/news/rss.xml">"#, r#"<a href="/news/rss.xml">"#),
            (r#"<a href="/video/vshdemo.mp4">"#, r#"<a href="/video/vshdemo.mp4">"#),
            (r#"<a href="/static/x/eu-short.at3">"#, r#"<a href="/static/x/eu-short.at3">"#),
            (r#"<link href="/static/css/all.css?v=abc">"#, r#"<link href="/static/css/all.css?v=abc">"#),
            (r#"<a href="/api/login">"#, r#"<a href="/api/login">"#),
            (r#"<a href="//example.com/x">"#, r#"<a href="//example.com/x">"#),
            (r#"<a href="https://example.com/x">"#, r#"<a href="https://example.com/x">"#),
        ];
        for (input, expected) in cases {
            assert_eq!(add_trailing_slashes(input), expected, "input: {input}");
        }
    }

    #[test]
    fn canonical_paths() {
        assert_eq!(canonical_path("/index"), "/");
        assert_eq!(canonical_path("/"), "/");
        assert_eq!(canonical_path("/news/foo"), "/news/foo/");
        assert_eq!(canonical_path("/news/foo/"), "/news/foo/");
    }
}

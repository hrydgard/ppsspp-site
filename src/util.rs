use anyhow::Context;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use serde_json::Value;

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
    let mut file = std::fs::File::create(file_path).context("create_file_as_dir")?;
    file.write_all(contents.as_bytes())?;
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
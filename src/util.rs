use anyhow::Context;
use std::fs;
use std::io::BufRead;
use std::path::Path;

pub fn copy_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_recursive(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn title_string(buffer: &mut impl BufRead) -> String {
    let mut first_line = String::new();
    let _ = buffer.read_line(&mut first_line);

    // Where do the leading hashes stop?
    let mut last_hash = 0;
    for (idx, c) in first_line.chars().enumerate() {
        if c != '#' {
            last_hash = idx;
            break;
        }
    }

    // Trim the leading hashes and any whitespace
    let first_line: String = first_line.drain(last_hash..).collect();
    let first_line = String::from(first_line.trim());

    first_line
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

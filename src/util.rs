use std::io::{BufRead, BufReader};
use std::path::Path;
use anyhow::Context;

pub fn title_string(path: &Path) -> String {
    // Read the first line of the file into `title`.
    let file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {:?}", &path),
    };
    let mut buffer = BufReader::new(file);
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
        std::fs::create_dir_all(parent).context("create_dir")
    } else {
        Ok(())
    }
}

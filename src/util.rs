use anyhow::Context;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn filename_to_string(name: &OsStr) -> String {
    // name.to_str().unwrap().to_owned()
    name.to_string_lossy().to_string()
}

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

use anyhow::Context;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn filename_to_string(name: &OsStr) -> String {
    // name.to_str().unwrap().to_owned()
    name.to_string_lossy().to_string()
}

#[allow(clippy::single_match)]
pub fn copy_recursive(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    minify: bool,
) -> anyhow::Result<()> {
    let minify_session = minify_js::Session::new();
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_recursive(entry.path(), dst.as_ref().join(entry.file_name()), minify)
                .context("copy-recurse")?;
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

pub fn concat_files(parent: &Path, inputs: &[&str], output: &str) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(parent.join(output))?;
    for input in inputs {
        let data = std::fs::read(parent.join(input))?;
        file.write_all(&data)?;
    }
    Ok(())
}

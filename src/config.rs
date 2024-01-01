use std::path::PathBuf;

pub struct Config {
    pub indir: PathBuf,
    pub outdir: PathBuf,
    pub markdown_options: markdown::Options,
}

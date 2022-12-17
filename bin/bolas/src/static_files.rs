use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub(crate) async fn serve_index_css(_req: HttpRequest) -> Result<NamedFile> {
    let index_css: PathBuf = "static/index.css".parse().unwrap();
    Ok(NamedFile::open(index_css)?)
}

pub(crate) async fn serve_index_html(_req: HttpRequest) -> Result<NamedFile> {
    let index_html: PathBuf = "static/index.html".parse().unwrap();
    Ok(NamedFile::open(index_html)?)
}

pub(crate) async fn serve_index_js(_req: HttpRequest) -> Result<NamedFile> {
    let index_js: PathBuf = "static/index.js".parse().unwrap();
    Ok(NamedFile::open(index_js)?)
}

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref INDEX_CSS: PathBuf = "static/index.css".parse().unwrap();
    static ref INDEX_HTML: PathBuf = "static/index.html".parse().unwrap();
    static ref INDEX_JS: PathBuf = "static/index.js".parse().unwrap();
}

pub(crate) async fn serve_index_css(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(&*INDEX_CSS)?)
}

pub(crate) async fn serve_index_html(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(&*INDEX_HTML)?)
}

pub(crate) async fn serve_index_js(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(&*INDEX_JS)?)
}

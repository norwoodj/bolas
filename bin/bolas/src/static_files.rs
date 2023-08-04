use actix_files::NamedFile;
use actix_web::{web, Result};
use serde::Deserialize;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub(crate) struct StaticFilePathParam {
    filename: String,
}

pub(crate) async fn serve_index_html(static_file_folder: web::Data<PathBuf>) -> Result<NamedFile> {
    do_serve_static_file(&static_file_folder, "index.html").await
}

pub(crate) async fn serve_static_file(
    static_file_folder: web::Data<PathBuf>,
    static_file_path_param: web::Path<StaticFilePathParam>,
) -> Result<NamedFile> {
    do_serve_static_file(&static_file_folder, &static_file_path_param.filename).await
}

pub(crate) async fn do_serve_static_file(
    static_file_folder: &Path,
    filename: &str,
) -> Result<NamedFile> {
    // Ensure that only files within the static directory can be served
    let filename = Path::new(filename).file_name().unwrap();
    let full_path = static_file_folder.join(filename);

    if !full_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "file not found").into());
    }

    NamedFile::open(full_path).map_err(Into::into)
}

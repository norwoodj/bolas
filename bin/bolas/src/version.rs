use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct VersionInfo {
    build_timestamp: String,
    revision: String,
    version: String,
}

pub(crate) fn load_version_file(path: &PathBuf) -> Option<VersionInfo> {
    let version_file = match File::open(path) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("Failed to open version file at path {:?}: {}", path, e);
            return None;
        }
    };

    match serde_json::from_reader(&version_file) {
        Ok(v) => Some(v),
        Err(e) => {
            log::warn!(
                "Failed to deserialize version info from version file at path {:?}: {}",
                path,
                e
            );
            None
        }
    }
}

pub(crate) async fn serve_version_info(
    version_info: web::Data<Option<VersionInfo>>,
) -> HttpResponse {
    let Some(v) = version_info.as_ref() else {
        return HttpResponse::NotFound().body(());
    };

    HttpResponse::Ok().json(v)
}

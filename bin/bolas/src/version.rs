use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct VersionInfo {
    build_timestamp: &'static str,
    git_revision: &'static str,
    version: &'static str,
}

impl Default for VersionInfo {
    fn default() -> Self {
        Self {
            build_timestamp: env!("VERGEN_BUILD_TIMESTAMP"),
            git_revision: env!("VERGEN_GIT_SHA"),
            version: env!("VERGEN_GIT_DESCRIBE"),
        }
    }
}

pub(crate) async fn serve_version_info(version_info: web::Data<VersionInfo>) -> HttpResponse {
    HttpResponse::Ok().json(&version_info)
}

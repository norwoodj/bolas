use actix_web::{HttpResponse, Result};
use lazy_static::lazy_static;
use prometheus::{register_int_counter, register_int_gauge, IntCounter, IntGauge, TextEncoder};

lazy_static! {
    pub(crate) static ref ARENAS_TOTAL: IntCounter =
        register_int_counter!("bolas_arenas_total", "Number of bolas arenas created").unwrap();
    pub(crate) static ref BOLAS_TOTAL: IntCounter =
        register_int_counter!("bolas_bolas_total", "Number of bolas created").unwrap();
    pub(crate) static ref ARENAS_ACTIVE: IntGauge = register_int_gauge!(
        "bolas_arenas_active",
        "Number of bolas arenas currently active"
    )
    .unwrap();
    pub(crate) static ref BOLAS_ACTIVE: IntGauge =
        register_int_gauge!("bolas_bolas_active", "Number of bolas currently active").unwrap();
}

pub async fn metrics_handler() -> Result<HttpResponse> {
    let mut body = String::new();
    let encoder = TextEncoder;
    encoder
        .encode_utf8(&prometheus::default_registry().gather(), &mut body)
        .unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(body))
}

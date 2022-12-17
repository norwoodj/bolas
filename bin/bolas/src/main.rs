use actix_web::{middleware::Logger, web, App, HttpServer};

mod bolas;
mod static_files;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(static_files::serve_index_html))
            .route("/index.js", web::get().to(static_files::serve_index_js))
            .route("/index.css", web::get().to(static_files::serve_index_css))
            .route("/ws", web::get().to(websocket::serve_websockets))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

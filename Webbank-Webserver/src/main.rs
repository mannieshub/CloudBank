use actix_web::{App, HttpServer};
use actix_cors::Cors;
use crate::routes::{account_routes, deposit_routes, withdraw_routes, transfer_routes, delete_history_routes, change_name_routes};

mod routes;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::default()
        .send_wildcard()
        .allow_any_origin()
        //.allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_any_method()
        //.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allow_any_header()

        
    )
            .configure(account_routes::config)
            .configure(deposit_routes::config)
            .configure(withdraw_routes::config)
            .configure(transfer_routes::config)
            .configure(delete_history_routes::config)
            .configure(change_name_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

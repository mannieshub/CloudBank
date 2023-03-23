use actix_web::{App ,HttpServer};
pub mod routes;
pub mod handlers;
mod models;
use crate::routes::{account_routes,deposit_routes,withdraw_routes,transfer_routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> { //Entry Point หรือจุดที่โปรแกรมเริ่มทำงาน
   HttpServer::new(|| {
       App::new()
            .configure(account_routes::config)
            .configure(deposit_routes::config)
            .configure(withdraw_routes::config)
            .configure(transfer_routes::config)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}

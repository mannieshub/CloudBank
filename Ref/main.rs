use actix_web::{middleware, App, HttpServer,get,Responder,HttpResponse};
use env_logger::Env;

mod handlers;
mod models;
pub mod routes;
use crate::routes::{
    money_routes,
    addmoney_routes,
    deletemoney_routes,
    editmoney_routes, 
    profilemoney_routes,
    todaymoney_routes,
};
#[get("/")]
async fn index() -> impl Responder {
   HttpResponse::Ok().body("Hello Welcome to web-Keptang!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .configure(profilemoney_routes::config)
            .configure(money_routes::config)
            .configure(todaymoney_routes::config)
            .configure(addmoney_routes::config)
            .configure(editmoney_routes::config)
            .configure(deletemoney_routes::config)
            
    })
    .bind("0.0.0.0:8080")?      // กรณีบิ้ว
    .run()
    .await
}

use actix_web::{get,Responder,HttpResponse};

#[get("/deposit")]
async fn deposit_money() -> impl Responder{
    HttpResponse::Ok().json("deposit")
}
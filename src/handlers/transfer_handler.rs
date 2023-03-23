use actix_web::{get,Responder,HttpResponse};

#[get("/transfer")]
async fn transfer_money() -> impl Responder{
    HttpResponse::Ok().json("Transfer Service")
}
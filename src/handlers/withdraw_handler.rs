use actix_web::{get,Responder,HttpResponse};

#[get("/withdraw")]
async fn withdraw_money() -> impl Responder{
    HttpResponse::Ok().json("Withdraw Service")
}
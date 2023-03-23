use actix_web::{get,Responder,HttpResponse};

#[get("/account")]
async fn account_id() -> impl Responder{
    HttpResponse::Ok().json("Account")
}
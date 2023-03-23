use actix_web::{get,Responder,HttpResponse};

#[get("/withdraw")]
async fn withdraw_money() -> impl Responder{
    HttpResponse::Ok().json("Withdraw Service")
}

//ผู้ใช้ใส่ id และจำนวนเงินที่ต้องการถอน
//update เงินในบัญชีของผู้ใช้ (-)
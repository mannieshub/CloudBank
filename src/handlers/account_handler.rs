use actix_web::{get,Responder,HttpResponse};

#[get("/account")]
async fn account_id() -> impl Responder{
    HttpResponse::Ok().json("Account")
}

//ให้ user พิม id เพื่อเช็คละแสดงข้อมูลที่มี id ตรงกัน
//ในกรณีที่เชื่อม db ก็เอา id ไปเช็คใน db ละแสดงผล
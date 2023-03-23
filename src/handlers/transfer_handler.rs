use actix_web::{get,Responder,HttpResponse};

#[get("/transfer")]
async fn transfer_money() -> impl Responder{
    HttpResponse::Ok().json("Transfer Service")
}

//ผู้ใช้กรอก id ตัวเองและ id ที่ต้องการโอน
//กรอกจำนวนเงินที่ต้องการโอน
//ทำการ update เงินของผู้ส่งและผู้รับ ผู้ส่ง(-) ผู้รับ(+)
//id ทั้ง 2 ต้องมีใน db หรือในไฟล์นี้หรือแสดงโง่ๆก็ได้
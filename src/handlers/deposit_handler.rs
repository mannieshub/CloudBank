use actix_web::{get,Responder,HttpResponse};

#[get("/deposit")]
async fn deposit_money() -> impl Responder{
    HttpResponse::Ok().json("Deposit Service")
}

//ให้ผู้ใช้ใส่ id และจำนวนเงินที่ต้องการฝาก
//จำนวนเงินจะถูกแก้ไขตามจำนวนเงินที่ฝาก **ไม่สามารถใส่เงินติดลบได้
//ในกรณียังเชื่อม db ไม่ได้ให้ใช้ + เงินที่ฝาก
//กรณ๊เชื่อม db ก็ใช้การ update น่าจะได้มั้ยไม่แน่ใจ เอาเงินต้น+เงินฝาก ละ update
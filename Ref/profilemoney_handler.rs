use actix_web::{ get, HttpResponse, Responder, web};
use log::{info};
use serde::{Serialize, Deserialize};
use crate::models::moneylist::*;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}
// GET /profile 
#[get("/profile")]
async fn get_profile(user_id: web::Json<UserdataUpgate>) -> impl Responder {
    info!("Keptang profile");

    // ค่า id ที่รับมา
    let userdata = user_id.into_inner();
    let id: i32 = userdata.id;

    // ค่าเริ่มต้น
    let mut user_money: i32 = 0;
    let mut user_name = "";

    if id==40956 {
        user_money = 115000;
        user_name = "vivat";
    }

    // ค่า หลังเช็คแล้ว
    let money_total = 
        Userdata{
            id: id,
            balancetotal: user_money,
            name: user_name.to_string(),
        };

    HttpResponse::Ok().json(money_total)
}
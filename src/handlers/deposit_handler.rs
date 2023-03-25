use crate::models::bank::{DepositMoney, DataDeposit};
use actix_web::{post, web, HttpResponse, Responder};
//use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
struct Headers {
    account_id: i32,
    amount: i32,
}
#[post("/deposit")]
async fn deposit_money(deposit: web::Json<Headers>) -> impl Responder {
    //HttpResponse::Ok().json("Deposit Service")
    let inner = deposit.into_inner();
    let id = inner.account_id;
    let amt = inner.amount;
    if amt <= 0 {
        return HttpResponse::BadRequest().json(json!({
            "message": "Invalid amount",
            "data": null
        }));
    }
    if id == 1212312121 {
        let deposit = DataDeposit {
            account_id: id,
            account_name: "Man".to_string(),
            available_balance: amt + 1000000000,
            time: "2023-03-16T15:00:00Z".to_string(),
        };

        let deposit_money = DepositMoney {
            message: "Deposit Success!".to_string(),
            data: vec![deposit],
        };
        HttpResponse::Ok().json(deposit_money)
    } else {
        HttpResponse::NotFound().json(json!({
            "message": "The request was invalid",
            "data": null
        }))
    }
}

//ให้ผู้ใช้ใส่ id และจำนวนเงินที่ต้องการฝาก
//จำนวนเงินจะถูกแก้ไขตามจำนวนเงินที่ฝาก **ไม่สามารถใส่เงินติดลบได้
//ในกรณียังเชื่อม db ไม่ได้ให้ใช้ + เงินที่ฝาก
//กรณ๊เชื่อม db ก็ใช้การ update น่าจะได้มั้ยไม่แน่ใจ เอาเงินต้น+เงินฝาก ละ update

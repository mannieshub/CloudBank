use crate::models::bank::{DataDeposit, DepositMoney};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
//use actix_cors::Cors;
use chrono::{DateTime, Utc};

//use http::header;
#[derive(Serialize, Deserialize)]

struct Headers {
    amount: i32,
}
#[post("/deposit/{id}")]
async fn deposit_money(account_id: web::Path<i32>, deposit: web::Json<Headers>) -> impl Responder {
    //HttpResponse::Ok().json("Deposit Service")

    let now: DateTime<Utc> = Utc::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let inner = deposit.into_inner();
    let id = account_id.to_string().parse().unwrap();
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
            time: formatted,
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

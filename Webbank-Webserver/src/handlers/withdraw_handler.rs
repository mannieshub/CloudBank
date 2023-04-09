use crate::models::bank::{WithdrawMoney, DataWithdraw};
use actix_web::{post, web, HttpResponse, Responder};
//use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]

struct Headers {
    amount: i32,
}

#[post("/withdraw/{id}")]
async fn withdraw_money(account_id: web::Path<i32>,withdraw: web::Json<Headers>) -> impl Responder{
   // HttpResponse::Ok().json("Withdraw Service")
    let now: DateTime<Utc> = Utc::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

   let inner = withdraw.into_inner();
    let id = account_id.to_string().parse().unwrap();
    let amt = inner.amount;
    if amt <= 0 {
        return HttpResponse::BadRequest().json(json!({
            "message": "Invalid amount",
            "data": null
        }));
    }
    if id == 1212312121 {
        let withdraw = DataWithdraw {
            message: format!("Withdraw {} THB success!", amt),
            account_id: id,
            account_name: "Man".to_string(),
            available_balance: 1000000000 - amt,
            time: formatted,
        };

        let withdraw_money = WithdrawMoney {
            message: "Withdraw Success!".to_string(),
            data: vec![withdraw],
        };
        HttpResponse::Ok().json(withdraw_money)
    } else {
        HttpResponse::NotFound().json(json!({
            "message": "The request was invalid",
            "data": null
        }))
    }
}

//ผู้ใช้ใส่ id และจำนวนเงินที่ต้องการถอน
//update เงินในบัญชีของผู้ใช้ (-)
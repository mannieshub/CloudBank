use actix_web::{get,web,Responder,HttpResponse};
//use log::{info, debug};
use serde_json::json;
use crate::models::bank::{AccountDetail,DataAccount,Transaction};
//use serde::{Serialize, Deserialize};
//use mysql::*;
//use mysql::prelude::*;
use crate::HttpServer;
use crate::App;
//use std::collections::HashMap;


#[get("/account/{id}")]
async fn account_id(account_id: web::Path<i32>) -> impl Responder{
    //HttpResponse::Ok().json("Account")
    //info!("Test");
    //debug!("Test");
    let id:i32 = account_id.to_string().parse().unwrap();
    
    if id == 1212312121 {
        let account = DataAccount {
            account_id: id,
            account_name: "Man".to_string(),
            saving_plans: "Saving Account".to_string(),
            available_balance: 1000000000,
            transaction_history: vec![
                Transaction {
                    time: "2023-03-16T15:00:00Z".to_string(),
                    transaction: "Transfer".to_string(),
                    transaction_id: 3,
                    to_id: "10101010".to_string(),
                    amount: 50000000,
                },
                Transaction {
                    time: "2023-03-16T15:00:00Z".to_string(),
                    transaction: "Transfer".to_string(),
                    transaction_id: 3,
                    to_id: "10101010".to_string(),
                    amount: 500000,
                },
                Transaction {
                    time: "2023-03-16T12:00:00Z".to_string(),
                    transaction: "Deposit".to_string(),
                    transaction_id: 2,
                    to_id: "-".to_string(),
                    amount: 1000000,
                },
                Transaction {
                    time: "2023-03-15T13:00:00Z".to_string(),
                    transaction: "Withdraw".to_string(),
                    transaction_id: 1,
                    to_id: "-".to_string(),
                    amount: 20000,
                },
            ],
    
        };
        
        let account_detail = AccountDetail {
            message: "Account Detail".to_string(),
            data: vec![account],
        };
        
        
        HttpResponse::Ok()
        
        .json(account_detail)
        
    } else {
        HttpResponse::NotFound()
        
        .json(json!({
            "message": "Account not found",
            "data": null
        }))

    }
    

    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ค่าโฮสต์และพอร์ท
    let addr = "127.0.0.1:8080";

    // เปิดเซิร์ฟเวอร์
    let server = HttpServer::new(|| {
        App::new()
            .service(account_id)
    })
    .bind(addr)?;

    server.run().await
}


//ให้ user พิม id เพื่อเช็คละแสดงข้อมูลที่มี id ตรงกัน
//ในกรณีที่เชื่อม db ก็เอา id ไปเช็คใน db ละแสดงผล
use crate::models::bank::{DeleteTransaction,DataDelete,DeleteDetail}; //bank.rs
use actix_web::{delete,web,HttpResponse,Responder};
use serde::{Serialize,Deserialize};
use serde_json::json;
use crate::HttpServer;
use crate::App;


#[delete("/account/history/{id}")]
async fn delete(account_id: web::Path<i32>)  -> impl  Responder {
    
    let id = account_id.to_string().parse().unwrap();
    if id == 1212312121 {
        let account = DataDelete {
            account_id: id,
            account_name: "Man".to_string(),
            saving_plans: "Saving Account".to_string(),
            available_balance: 1000000000,
            transaction_history: vec![
                DeleteTransaction {
                    time: "2023-03-16T15:00:00Z".to_string(),
                    transaction: "Transfer".to_string(),
                    transaction_id: 3,
                    to_id: "10101010".to_string(),
                    amount: 50000000,
                },
                DeleteTransaction {
                    time: "2023-03-16T15:00:00Z".to_string(),
                    transaction: "Transfer".to_string(),
                    transaction_id: 3,
                    to_id: "10101010".to_string(),
                    amount: 500000,
                },
                DeleteTransaction {
                    time: "2023-03-16T12:00:00Z".to_string(),
                    transaction: "Deposit".to_string(),
                    transaction_id: 2,
                    to_id: "-".to_string(),
                    amount: 1000000,
                },
                DeleteTransaction {
                    time: "2023-03-15T13:00:00Z".to_string(),
                    transaction: "Withdraw".to_string(),
                    transaction_id: 1,
                    to_id: "-".to_string(),
                    amount: 20000,
                },
            ],
    
        };
        
        let account_detail = DeleteDetail {
            message: "Delete Success!".to_string(),
            data: vec![account],
        };
        
        
        HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Methods", "DELETE, OPTIONS")
        .json(account_detail)
        
    } else {
        HttpResponse::NotFound()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .header("Access-Control-Allow-Methods", "GET, OPTIONS")
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
            .service(delete)
    })
    .bind(addr)?;

    server.run().await
}
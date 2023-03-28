use actix_web::{put,web,Responder,HttpResponse};
//use log::{info, debug};
use serde_json::json;
use crate::models::bank::{AccountDetail,DataAccount,Transaction};
use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize)]
pub struct Headers { 
    pub new_name : str
}

#[put("/account/{id}")]
async fn account_id(account_id: web::Path<i32>,change:web::Json<Headers>) -> impl Responder{
    let inner: Headers = change.into_inner();
    //HttpResponse::Ok().json("Account")
    //info!("Test");
    //debug!("Test");
    let id:i32 = account_id.to_string().parse().unwrap();
    let n_name:str= inner.new_name;

    if id == 1212312121 {
        let account = DataAccount {
            account_id: id,
            account_name: n_name.to_string(),
            saving_plans: "Saving Account".to_string(),
            available_balance: 1000000000,
            transaction_history: vec![
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
        HttpResponse::Ok().json(account_detail)
    } else {
        HttpResponse::NotFound().json(json!({
            "message": "Account not found",
            "data": null
        }))
    }
    

    
}

//ให้ user พิม id เพื่อเช็คละแสดงข้อมูลที่มี id ตรงกัน
//ในกรณีที่เชื่อม db ก็เอา id ไปเช็คใน db ละแสดงผล
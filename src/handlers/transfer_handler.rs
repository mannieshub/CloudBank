use crate::models::bank::{TransferMoney,DataTransfer};
use actix_web::{post,web,HttpResponse,Responder};

use serde::{Serialize,Deserialize};
use serde_json::json;

#[derive(Serialize,Deserialize)]

//ส่วนติดต่อ front-end
pub struct Headers { 
    pub destination_id: i32,
    pub amount: i32,
}

#[post("/transfer/{id}")]
async fn transfer_money(account_id:web::Path<i32>,transfer:web::Json<Headers>) -> impl Responder {
    let inner: Headers = transfer.into_inner();
    let id: i32 = account_id.to_string().parse().unwrap();

    let d_id:i32 = inner.destination_id;

    let amt: i32 = inner.amount;

    if amt <= 0 {
        return HttpResponse::BadRequest().json(json!({
            "message": "Invalid amount",
            "data": null
        }));
    }
            //ส่วนติดต่อ database//

    if id == 1212312121 {
        
        let transfer: DataTransfer = DataTransfer { 
            message: ("Transfer Success!").to_string(), 
            account_id: (id), 
            account_name: ("Man").to_string(), 
            to_id: (d_id), 
            to_name: ("TewNaja").to_string(), 
            available_balance: 1_000_000_000 -  amt, 
            time: ("2023-03-16T15:00:00Z").to_string(),
        };

        let transfer_money: TransferMoney = TransferMoney { 
            message: ("Transfer Success!").to_string(), 
            data: vec![transfer],
         };
         HttpResponse::Ok().json(transfer_money)
    } 
    else {
            HttpResponse::NotFound().json(json!({
                "message": "User not found!",
                "data": null
            }))
    }

}
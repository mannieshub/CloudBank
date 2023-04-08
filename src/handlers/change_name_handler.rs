use crate::models::bank::{ChangeName, DataChange}; //bank.rs
use actix_web::{put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
struct Headers {
    new_name: String,
}
#[put("/change_name/{id}")]
async fn change_name(
    account_id: web::Path<i32>,
    change_name: web::Json<Headers>,
) -> impl Responder {
    let inner: Headers = change_name.into_inner();
    let id = account_id.to_string().parse().unwrap();
    let name = inner.new_name.to_string();

    if id == 1212312121 {
        let y: DataChange = DataChange {
            // models/bank.rs
            message: ("Edit Success!").to_string(),
            account_id: id,
            account_name: name.to_string(),
        };
        let change_name = ChangeName {
            // models/bank.rs
            message: ("Edit account_name").to_string(),
            data: vec![y],
        };
        HttpResponse::Ok()
           
            .json(change_name)
    } else {
        HttpResponse::NotFound()
            
            .json(json!({
                "message": "User not found!",
                "data": null
            }))
    }
}

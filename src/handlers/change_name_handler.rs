use crate::models::bank::{}; //bank.rs
use actix_web::{put,web,HttpResponse,Responder};
use serde::{Serialize,Deserialize};
use serde_json::json;
#[derive(Serialize,Deserialize)]
pub struct Headers {
    pub account_id:i32,
    pub new_name:str,
}

#[put("/change_name/{id}")]
async fn change_name(account_id:web::Path<i32>,changename:web::Json<Headers>)  -> impl  Responder {
    let inner: Headers = changename.into_inner();
    let id:i32 = account_id.to_string().parse().unwrap();
    let name:str = inner.new_name.to_string();

    if id == 1212312121 {

        let x : y = y {  // models/bank.rs
            message:("Edit name success").to_string(),
            account_id:id,
            account_name:name.to_string(),
            };
            HttpResponse::Ok().json(change_name)
        }
        else {
            HttpResponse::NotFound().json(json!({
                "message": "User not found!",
                "data": null
            }))
    }
    }
}
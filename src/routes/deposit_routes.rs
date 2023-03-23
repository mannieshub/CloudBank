use actix_web::{web};
use crate::handlers::deposit_handler::{deposit_money};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
    .service(deposit_money);
}
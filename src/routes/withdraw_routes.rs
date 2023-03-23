use actix_web::{web};
use crate::handlers::withdraw_handler::{withdraw_money};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
    .service(withdraw_money);
}
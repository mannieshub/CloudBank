use actix_web::{web};
use crate::handlers::transfer_handler::transfer_money;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(transfer_money);
}
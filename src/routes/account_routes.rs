use actix_web::{web};
use crate::handlers::account_handler::account_id;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(account_id);
}
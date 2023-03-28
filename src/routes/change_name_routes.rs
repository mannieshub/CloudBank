use actix_web::{web};
use crate::handlers::change_name_handler::{account_id};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
    .service(account_id);
}
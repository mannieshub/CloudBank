use actix_web::{web};
use crate::handlers::change_name_handler::{change_name};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
    .service(change_name);
}
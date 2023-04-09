use actix_web::{web};
use crate::handlers::delete_history_handler::{delete};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
    .service(delete);
}
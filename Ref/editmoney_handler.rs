use crate::models::moneylist::*;
use actix_web::{put, web, HttpResponse};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}

#[derive(Serialize, Deserialize)]
struct MoneylistUpgate {
    description: String,
    date: String,
    amount: i32,
    types: String,
}
#[derive(Serialize, Deserialize)]
struct PostMoneyRequest {
    user_data: UserdataUpgate,
    dataitem: MoneylistUpgate,
}

// PUT /money/edit/{id}: รับ JSON ที่มีค่าของคีย์ "expense"/"income" เป็นรายการ JSON ที่มีข้อมูลของรายการรายจ่ายที่ต้องการอัปเดตด้วย ID ที่ระบุ
#[put("/money/edit/{id}")]
async fn put_money(list_id: web::Path<i32>,input_data: web::Json<PostMoneyRequest>) -> HttpResponse {
    info!("put money by id");
    debug!("id: {} 🪄", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let _editdata = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();

    // สมมุติข้อมูลเดิมของ id 3
    let mut _editdata_old = Moneylist {
        list_id: 3,
        description: "แม่ให้".to_string(),
        date: "2023-03-15".to_string(),
        amount: 100,
        types: "income".to_string(),
    };

    let _editdata_new = Moneylist {
        list_id: id,
        description: _editdata.dataitem.description,
        date: _editdata.dataitem.date,
        amount: _editdata.dataitem.amount,
        types: _editdata.dataitem.types,
    };

    if id == 3 {
        debug!("มี list_id นี้จริง ✅");
    } else {
        _editdata_old = Moneylist {
            list_id: 0,
            description: "ไม่มี".to_string(),
            date: "ไม่บอก".to_string(),
            amount: 0,
            types: "ไม่นะ".to_string(),
        };
        debug!("ไม่มี list_id นี้จริง 🐤");
    }

    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
    #[derive(Serialize, Deserialize)]
    struct CombinedResponse {
        items_old: Moneylist,
        items_new: Moneylist,
        text: String,
    }

    let combined_response = CombinedResponse {
        items_old: _editdata_old,
        items_new: _editdata_new,
        text: "ทำการแก้ไขข้อมูลละเด้อ".to_string(),
    };

    let response_body = json!(combined_response);

    HttpResponse::Ok().json(response_body)
}
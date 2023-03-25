use actix_web::{delete, web, HttpResponse};
use log::{debug, info};
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::models::moneylist::*;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}

// DELETE /money/delete/{id}: ไม่ต้องการรับข้อมูลใดๆ แต่จะลบรายการรายจ่ายที่มีอยู่ด้วย list_id ที่ระบุ
#[delete("/money/delete/{id}")]
async fn delete_money(list_id: web::Path<i32>,input_data: web::Json<UserdataUpgate>) -> HttpResponse {
    info!("delete money by list_id");
    debug!("list_id: {} ❌", list_id);

    // ค่าเริ่มต้น ที่รับมาแบบ JSON (ถ้าอยากแก้ไข เติม mut หลัง let)
    let _editdata = input_data.into_inner();
    let id: i32 = list_id.to_string().parse().unwrap();

    // สมมุติข้อมูลเดิมของ id 3
    let mut _data_old = Moneylist {
        list_id: 3,
        description: "แม่ให้".to_string(),
        date: "2023-03-15".to_string(),
        amount: 100,
        types: "income".to_string(),
    };

    // สมมุติ จะลบ รายการที่ id 3
    if id == 3 {
        debug!("DELETE {} ❌", list_id);
    }else{  // สมมุติไม่มีใน ฐานข้อมูล
        debug!("none data list_id: {} ❌", list_id);
        _data_old = Moneylist {
            list_id: 0,
            description: "ไม่มีเด้อ".to_string(),
            date: "ไม่มีเด้อ-ไม่มีเด้อ-ไม่มีเด้อ".to_string(),
            amount: 0,
            types: "ไม่มีเด้อ".to_string(),
        };
    }


    // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
    #[derive(Serialize, Deserialize)]
    struct CombinedResponse {
        items: Moneylist,
        text: String,
    }

    let combined_response = CombinedResponse {
        items: _data_old,
        text: "ทำการลบข้อมูลละเด้อ".to_string(),
    };

    let response_body = json!(combined_response);


    HttpResponse::Ok().json(response_body)
}

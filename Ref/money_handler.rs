use actix_web::{ get, HttpResponse, Responder, web};
use log::{ info};
use serde_json::json;
use serde::{Serialize, Deserialize};
use crate::models::moneylist::*;

// สร้าง struct ใหม่ที่มีเฉพาะส่วนที่คุณต้องการส่ง
#[derive(Serialize, Deserialize)]
struct UserdataUpgate {
    id: i32,
}

// GET /money: สำหรับอ่านข้อมูลรายการรายรับรายจ่ายทั้งหมด
#[get("/money")]
async fn get_money(user_id: web::Json<UserdataUpgate>) -> impl Responder {
    info!("Keptang money");

    // ค่า id ที่รับมา
    let userdata = user_id.into_inner();
    let id: i32 = userdata.id;
    // ค่าเริ่มต้น
    let mut _user_money: i32 = 0;
    let mut money_item_all = vec![];
    let mut _user_name = "";
    // กรณีเช็ค database นี่แค่ตัวอย่าง
    if id==40956 {
        _user_money = 115000;
        _user_name = "vivat";
        money_item_all = vec![
        Moneylist {
             list_id: 5,
             description: "เลี้ยงข้า".to_string(),
             date: "2023-03-15".to_string(),
             amount: 100,
             types: "expense".to_string(),
        },
        Moneylist {
            list_id: 4,
            description: "ซื้อข้าวเช้า".to_string(),
            date: "2023-03-15".to_string(),
            amount: 100,
            types: "expense".to_string(),
       },
       Moneylist {
        list_id: 3,
        description: "แม่ให้".to_string(),
        date: "2023-03-15".to_string(),
        amount: 300,
        types: "income".to_string(),
        },
        Moneylist {
            list_id: 2,
            description: "สาวเลี้ยง".to_string(),
            date: "2023-03-14".to_string(),
            amount: 5000,
            types: "income".to_string(),
        },
        Moneylist {
            list_id: 1,
            description: "นิวจีนบั้มใหม่".to_string(),
            date: "2023-03-13".to_string(),
            amount: 5000,
            types: "expense".to_string(),
        },     
    ];
    }else{
        _user_money = 0;
        _user_name = "noname";
    }
    // ค่า หลังเช็คแล้ว
    let money_total = 
        Userdata{
            id: id,
            balancetotal: _user_money,
            name: _user_name.to_string(),
        };
   // สร้างโครงสร้างข้อมูลสำหรับรวมผลลัพธ์
   #[derive(Serialize)]
   struct CombinedResponse {
       total: Userdata,
       items: Vec<Moneylist>,
   }

   let combined_response = CombinedResponse {
       total: money_total,
       items: money_item_all,
   };
   
   let response_body = json!(combined_response);
    
    HttpResponse::Ok().json(response_body)
}



use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct LoginSystem {
    pub account_id: i32,
    pub password: String,
}

#[derive(Serialize,Deserialize)]
pub struct RegisterSystem {
    pub username: String,
    pub password: String,
    pub savingplan: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub time: String,
    pub transaction: String,
    pub transaction_id: i32,
    pub to_id: String,
    pub amount: i32,
}

#[derive(Serialize, Deserialize)]
pub struct DataAccount {
    pub account_id: i32,
    pub account_name: String,
    pub saving_plans: String,
    pub available_balance: i32,
    pub transaction_history: Vec<Transaction>,
}

#[derive(Serialize,Deserialize)]
pub struct AccountDetail {
    pub message: String,
    pub data: Vec<DataAccount>,
    
}

#[derive(Serialize, Deserialize)]
pub struct DataDeposit {
    pub account_id: i32,
    pub account_name: String,
    pub available_balance: i32,
    pub time: String,
}

#[derive(Serialize,Deserialize)]
pub struct DepositMoney {
    pub message: String,
    pub data: Vec<DataDeposit>,
    
}

#[derive(Serialize, Deserialize)]
pub struct DataWithdraw {
    pub message: String,
    pub account_id: i32,
    pub account_name: String,
    pub available_balance: i32,
    pub time: String,
}

#[derive(Serialize,Deserialize)]
pub struct WithdrawMoney {
    pub message: String,
    pub data: Vec<DataWithdraw>,
    
}

#[derive(Serialize, Deserialize)]
pub struct DataTransfer {
    pub message: String,
    pub account_id: i32,
    pub account_name: String,
    pub to_id: i32,
    pub to_name: String,
    pub available_balance: i32,
    pub time: String,
}

#[derive(Serialize,Deserialize)]
pub struct TransferMoney {
    pub message: String,
    pub data: Vec<DataTransfer>,
    
}
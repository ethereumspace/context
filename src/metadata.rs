use ic_cdk::{
    export::Principal
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    canister:String,
    caller: String,
    fee: u64,
    method_name: String,
    transaction_time: u64,
    stable_size: u64,
}


impl  Metadata {
    pub fn new(canister:&Principal,caller: &Principal, transaction_time: u64, stable_size: u64,fee:u64,method_name:&str) -> Metadata {
        Metadata {
            canister:canister.to_text(),
            caller: caller.to_text(),
            fee: fee,
            method_name: method_name.to_string(),
            transaction_time: transaction_time,
            stable_size: stable_size,
        }
    }
    
    pub fn get_canister(&self) -> &String{
         &self.canister
    }
}


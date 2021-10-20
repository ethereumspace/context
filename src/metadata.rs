use ic_cdk::{
    export::Principal
};

use serde::{Deserialize, Serialize};
use ic_cdk::export::candid::CandidType;
#[derive(Serialize, Deserialize, Debug,Clone,CandidType)]
pub struct Metadata {
    pub canister:String,
    pub caller: String,
    pub fee: u64,
    pub method_name: String,
    pub transaction_time: u64,
    pub stable_size: u64,
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

}


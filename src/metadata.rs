use ic_cdk::{export::Principal};
use ic_cdk::export::candid::{Deserialize,Nat};
use ic_cdk::export::candid::CandidType;
#[derive( Deserialize, Debug,Clone,CandidType)]
pub struct Metadata {
    pub canister:String,
    pub caller: String,
    pub fee: Nat,
    pub method_name: String,
    pub transaction_time: Nat,
    pub stable_size: Nat,
}

impl  Metadata {
    pub fn new(canister:&Principal,caller: &Principal, transaction_time: Nat, stable_size: Nat,fee:Nat,method_name:&str) -> Metadata {
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



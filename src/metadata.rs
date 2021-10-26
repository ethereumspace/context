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


#[allow(non_camel_case_types)]
#[derive(CandidType, Debug, Deserialize)]
pub enum CanisterStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
}



#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct CanisterStatusResponse {
    pub status: CanisterStatus,
    pub settings: CanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub controller: Principal,
    pub memory_size: Nat,
    pub cycles: Nat,
}

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}
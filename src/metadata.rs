use ic_cdk::{export::Principal};
use ic_cdk::export::candid::{Deserialize,Nat};
use ic_cdk::export::candid::CandidType;

#[derive( Deserialize, Debug,Clone,CandidType)]
pub struct Metadata {
    pub canister:Principal,
    pub caller: Principal,
    pub cycle: Nat,
    pub method_name: String,
    pub transaction_time: Nat,
    pub stable_size: Nat,
}


impl Metadata {
    pub fn new(canister:&Principal,caller: &Principal, transaction_time: Nat, stable_size: Nat,cycle:Nat,method_name:&str) -> Self {
        Self {
            canister:canister.clone(),
            caller: caller.clone(),
            cycle: cycle,
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

#[derive(CandidType, Deserialize)]
pub enum InstallMode {
    #[serde(rename = "install")]
     Install,
    #[serde(rename = "reinstall")]
     Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterInstall {
    pub  mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub arg: Vec<u8>,
}

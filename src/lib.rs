pub mod metadata;
pub mod eventerr;
pub mod util;
mod config;
use config::{CREATETRANSACTION,STORAGECANISTER};
use metadata::{ Metadata};
use eventerr::{ EventErr};
use ic_cdk::{id,api::{stable, time,canister_balance},caller,print};
use ic_cdk::api::call::{ CallResult};


#[inline(always)]
pub async fn emit() -> () {
    // let name = ic_cdk::api::call::method_name();
    print("emit");
    let canister = id();
    let caller = caller();
    let transaction_time = time();
    let fee = canister_balance();
    let stable_size = stable::stable_size();
    let data = Metadata::new(&canister,&caller, transaction_time.into(), stable_size.into(),fee.into(),"test");
    let p = ic_cdk::export::Principal::from_text(STORAGECANISTER).unwrap();
    let res:CallResult<()> =  ic_cdk::api::call::call(p,CREATETRANSACTION,(&data,)).await;
}
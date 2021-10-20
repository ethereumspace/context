pub mod metadata;
pub mod eventerr;
pub mod util;
use metadata::{ Metadata};
use eventerr::{ EventErr};
use ic_cdk::{id,api::{stable, time},caller};
use ic_cdk::api::call::{ CallResult};

pub async fn emit() -> () {
    let canister = id();
    let caller = caller();
    let transaction_time = time();
    let stable_size = stable::stable_size();
    let data = Metadata::new(&canister,&caller, transaction_time.into(), stable_size.into(),10.into(),"test");
    let p = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let res:CallResult<()> =  ic_cdk::api::call::call(p, "storage",(&data,)).await;
}



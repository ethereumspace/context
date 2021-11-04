
pub mod metadata;
pub mod eventerr;
pub mod util;
pub mod canister;
mod config;
use config::{CREATETRANSACTION,EVENTCANISTER};
use metadata::{ Metadata};
use ic_cdk::export::Principal;
use ic_cdk::{id,api::{stable, time,canister_balance},caller,print};
use ic_cdk::api::call::{ CallResult};


#[inline(always)]
pub async fn emit(remarks: &str) -> CallResult<()>{
    let method_name  = ic_cdk::api::call::method_name();
    print("*ddddddd")
    let canister_id = id();
    let caller_id = caller();
    let event_time = time();
    let cycle = canister_balance();
    let stable_size = stable::stable_size();
    let new_metadata = Metadata::new(&canister_id,&caller_id, event_time.into(), stable_size.into(),cycle.into(),&method_name);
    match Principal::from_text(EVENTCANISTER) {
        Ok(event_canister_id) => {
            print("*ddddddd2");
            return ic_cdk::api::call::call(event_canister_id,CREATETRANSACTION,(&new_metadata,)).await;
        },
        Err(err) =>{
            print("*ddddddd3");
          return  Err((202.into(),err.to_string()))
        }
    } 
}


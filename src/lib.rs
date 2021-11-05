
pub mod metadata;
pub mod eventerr;
pub mod util;
pub mod canister;

mod config;
use config::{CREATETRANSACTION,EVENTCANISTER};
use metadata::{ Metadata};
use ic_cdk::api;
use ic_cdk::export::Principal;
use ic_cdk::api::call::{ CallResult,RejectionCode};
use event::EventTrait;
use event_macro::Event;
use ic_cdk::export::candid::Nat;
extern crate event;
extern crate event_macro;

pub async fn emit(event:impl EventTrait ) -> CallResult<()>{
    if event.method_name() == "" {
        return Err((RejectionCode::Unknown,"method_name is empty".to_string()));
    }
    if event.memo().len() > 30 {
        return Err((RejectionCode::Unknown,"memo is empty".to_string()));
    }
    let canister_id = event.canister_id();
    let caller_id = event.caller_id();
    let event_time = event.event_create_time();
    let cycle = event.canister_balance();
    let stable_size = event.stable_size();
    let method_name = event.method_name();
    let memo = event.memo();
    let new_metadata = Metadata::new(&canister_id,&caller_id, event_time.into(), stable_size.into(),cycle.into(),&method_name,&memo);
    match Principal::from_text(EVENTCANISTER) {
        Ok(event_canister_id) => {
           ic_cdk::api::call::call(event_canister_id,CREATETRANSACTION,(&new_metadata,)).await
        },
        Err(err) =>{
            Err((RejectionCode::Unknown,err.to_string()))
        }
    }
}




#[cfg(test)]
mod event_macro_test {
    use super::*;

    #[derive(Event)]
    struct EventTest{
        pub method_name:String,
        pub memo:String
    }
    #[test]
    fn event_trait_test(){
            let et = EventTest{
                method_name:"mint".to_string(),
                memo:"memo".to_string()
            };
            let method_name = et.method_name();
            println!("method_name {}",method_name);
            let memo = et.memo();
            println!("memo {}",memo);
    }
}
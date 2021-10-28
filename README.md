# context

  Context is to collect canisters information and store them to help developers better observe the running state of canisters
## example 
    
use ic_cdk_macros::*;
use ic_cdk::export::candid::{Nat};
use context::emit;
static mut CURRENT_SUPPLY:Option<Nat> = None ;

#[init]
fn init(){
    unsafe{
        CURRENT_SUPPLY = Some(Nat::default());
    }
}

#[update]
async fn mint(amount:Nat) ->() {
    unsafe{
        let v = CURRENT_SUPPLY.as_mut().unwrap();
        *v = v.clone() + amount;
   
    }
    emit().await;
}
#[post_upgrade]
fn post_upgrade(){
    unsafe{
        CURRENT_SUPPLY = Some(Nat::default());
    }
}
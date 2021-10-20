pub mod metadata;
pub mod eventerr;
use metadata::{ Metadata};
use eventerr::{ EventErr};
use ic_cdk::{id,api::{stable, time},caller, print,export::Principal};
use ic_cdk::api::call::{ CallResult};
pub mod generate {
    use super::EventErr;
    use super::Metadata;
    use std::result::Result;

    pub fn crateRecode(data: &Metadata) -> Result<String, EventErr> {
        let serial = serde_json::to_string(data).unwrap();
        if serial.len() == 0 {
            return Err(EventErr::SerializeFail);
        }
        Ok(serial)
    }
}

pub async fn emit() -> () {
    let canister = id();
    let caller = caller();
    let transaction_time = time();
    let stable_size = stable::stable_size();
    let data = Metadata::new(&canister,&caller, transaction_time, stable_size as u64,10 as u64,"test");

    let args= generate::crateRecode(&data).unwrap();
    let p = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let res:CallResult<()> =  ic_cdk::api::call::call(p, "storage",(&args,)).await;
}


#[cfg(test)]
mod mock{
    use std::collections::BTreeMap;
    #[test]
    pub fn demo(){
        let mut bt: BTreeMap::<u32,Vec<u32>>  = BTreeMap::new();
        bt.insert(1,vec![1,2]);
        bt.get_mut(&1).unwrap().push(3);
        let res = bt.get(&1).unwrap();
        println!("{:?}",res);
    }
}
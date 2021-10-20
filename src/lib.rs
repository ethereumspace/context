pub mod metadata;
pub mod eventerr;
use metadata::{ Metadata};
use eventerr::{ EventErr};
use ic_cdk::{id,api::{stable, time},caller, print,export::Principal};

pub mod generate {
    use super::EventErr;
    use super::Metadata;
    use std::result::Result;

    pub fn crateRecode(data: &Metadata) -> Result<Vec<u8>, EventErr> {
        let serial = serde_json::to_vec(data).unwrap();
        if serial.len() == 0 {
            return Err(EventErr::SerializeFail);
        }
        Ok(serial)
    }
}

pub async fn emit() -> () {
    // let canister = id();
    // let caller = caller();
    // let transaction_time = time();
    // let stable_size = stable::stable_size();
    // let data = Metadata::new(&canister,&caller, transaction_time, stable_size as u64,10 as u64,"test");
    // if let Err(EventErr) = generate::crateRecode(&data) {
    //     print("序列化出错了")
    // }
    let p = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    ic_cdk::call::<(), ()>(p, "storage",  ()).await;
}

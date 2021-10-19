#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use ic_cdk;
use ic_cdk::export::{Principal};
use ic_cdk::export::candid;
use candid::utils::{ArgumentDecoder, ArgumentEncoder};
use ic_cdk_macros::*;


pub async  fn createTransaction(id: ic_cdk::export::Principal, method: String) ->(){
    ic_cdk::print(ic_cdk::caller().to_text());
    let p = ic_cdk::export::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    ic_cdk::call::<(), ()>(p, "storage",  ()).await;
}


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

use ic_cdk_macros::*;

pub async  fn createTransaction(id: Principal, method: &str) {
    ic_cdk::print(ic_cdk::caller().to_text());
    ic_cdk::call::<(),()>(id,method,()).await;
}


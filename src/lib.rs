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
pub async  fn createTransaction<T:ArgumentEncoder>(id: Principal, method: &str, args: T) {
    ic_cdk::print(ic_cdk::caller().to_text());
    ic_cdk::call::<T,()>(id,method,args).await;
}

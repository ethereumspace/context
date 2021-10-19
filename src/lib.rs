#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
use ic_cdk;
pub fn createTransaction() {
    ic_cdk::print(ic_cdk::caller().to_text());
}

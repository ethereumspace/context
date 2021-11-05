use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
pub trait EventTrait {
    fn canister_id(&self) -> Principal;
    fn caller_id(&self) -> Principal;
    fn canister_balance(&self) -> Nat;
    fn event_create_time(&self) -> Nat;
    fn stable_size(&self) -> Nat;
    fn method_name(&self) -> String;
    fn memo(&self) -> String;
}

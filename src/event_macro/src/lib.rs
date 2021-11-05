extern crate proc_macro;

use crate::proc_macro::TokenStream;
use event::EventTrait;
use ic_cdk::api;
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use quote::quote;
use syn;

#[proc_macro_derive(Event)]
pub fn Event(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_event_macro(&ast)
}

fn impl_event_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EventTrait  for #name {
            fn canister_id(&self) -> Principal{
                api::id()
            }
            fn caller_id(&self) ->  Principal{
                api::caller()
            }
            fn canister_balance(&self) ->  Nat{
                api::canister_balance().into()
            }
           fn event_create_time(&self) -> Nat{
            api::time().into()
           }
           fn stable_size(&self) -> Nat{
            api::stable::stable_size().into()
           }
           fn method_name(&self) -> String{
            self.method_name.clone()
           }
           fn memo(&self) -> String{
            self.memo.clone()
           }
        }
    };
    gen.into()
}

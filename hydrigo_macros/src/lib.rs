#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn hydrigo(_item: TokenStream) -> TokenStream {
    dbg!(_item);
    quote!(
        { () }
    ).into()
}
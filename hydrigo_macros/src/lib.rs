#![crate_type = "proc-macro"]
extern crate proc_macro;
use syn::{parse_macro_input, DeriveInput, Data, Fields};
use proc_macro::TokenStream;
use quote::{quote, format_ident};

#[proc_macro]
pub fn hydrigo(item: TokenStream) -> TokenStream {
    dbg!(item);
    quote!(
        { () }
    ).into()
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let component = parse_macro_input!(item as DeriveInput);
    let struct_name = component.ident;
    let attr_name = format_ident!("{}Attribute", struct_name);
    let Data::Struct(component) = component.data else {
        panic!("a component must be a structure of named fields")
    };
    let Fields::Named(fields) = component.fields else {
        panic!("a component must be a structure of named fields")
    };
    let fields = fields.named;
    
    let mut fields_1 = Vec::new();
    fields.pairs().for_each(|field| {
        let name = field.value().ident.clone().unwrap();
        let ty = &field.value().ty;
        fields_1.extend(quote!(pub #name: #ty ,).into_iter());
    });

    let mut items_1 = Vec::new();
    fields.pairs().for_each(|field| {
        let name = field.value().ident.clone().unwrap();
        let set_ident = format_ident!("set_{}", name);
        let ty = &field.value().ty;
        items_1.extend(quote!(
            fn #set_ident (&mut self, value: #ty) {
                self.#name = value;
            }
        ).into_iter());
    });

    quote!(
        pub struct #struct_name {
            #(#fields_1)*
        }
        #[derive(Default)]
        pub struct #attr_name {
            #(#fields_1)*
        }
        impl #attr_name {
            #(#items_1)*
        }
    ).into()
}
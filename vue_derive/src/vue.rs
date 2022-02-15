use crate::{js_class, js_props, reactive, Input};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate(input: &DeriveInput) -> TokenStream {
    let input = match Input::from(input) {
        Some(input) => input,
        None => panic!("only struct are supported"),
    };

    let js_class = js_class::generate(&input);
    let js_props = js_props::generate(&input);
    let reactive = reactive::generate(&input);

    quote! {
        #js_class
        #js_props
        #reactive
    }
}

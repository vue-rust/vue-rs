mod attrs;
mod field;
mod input;
mod js_class;
mod js_props;
mod methods;
mod reactive;
mod vue;

use attrs::FieldAttrs;
use field::Field;
use input::Input;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn methods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    methods::generate(&input).into()
}

#[proc_macro_derive(Vue, attributes(vue))]
pub fn vue(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    vue::generate(&input).into()
}

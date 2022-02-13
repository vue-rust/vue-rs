mod attrs;
mod field;
mod input;
mod js_class;
mod js_create;
mod reactive;
mod vue;

use attrs::FieldAttrs;
use field::Field;
use input::Input;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Vue, attributes(vue))]
pub fn vue(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    vue::generate(&input).into()
}

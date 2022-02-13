use crate::Input;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate(input: &Input) -> TokenStream {
    let fields = &input.fields;
    let create_ident = input.js_create_ident();
    let create_js_name_lit = input.js_create_name_lit();

    let props = fields
        .iter()
        .filter(|f| f.attrs.prop || f.attrs.sync)
        .map(|f| f.camel_lit())
        .collect::<Vec<_>>();

    quote! {
        #[automatically_derived]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #create_js_name_lit)]
        pub fn #create_ident() -> wasm_bindgen::JsValue {

            wasm_bindgen::JsValue::from_serde(&vue::macros_utils::CreateConfig {
                props: vec![#(#props),*],
            }).expect("CreateConfig")
        }
    }
}

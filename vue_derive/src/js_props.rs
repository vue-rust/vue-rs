use crate::Input;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate(input: &Input) -> TokenStream {
    let fields = &input.fields;
    let props_ident = input.js_props_ident();
    let props_lit = input.js_props_lit();

    let props = fields
        .iter()
        .filter(|f| f.attrs.prop || f.attrs.sync)
        .map(|f| f.camel_lit())
        .collect::<Vec<_>>();

    quote! {
        #[automatically_derived]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #props_lit)]
        pub fn #props_ident() -> wasm_bindgen::JsValue {
            wasm_bindgen::JsValue::from_serde(&[#(#props),*]).expect(#props_lit)
        }
    }
}

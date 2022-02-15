use crate::Input;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate(input: &Input) -> TokenStream {
    let vis = input.vis;
    let props = input.ident;
    let reactive = input.reactive_ident();
    let fields = &input.fields;
    let types = fields.iter().map(|f| f.ty).collect::<Vec<_>>();
    let setters = fields.iter().map(|f| f.setter_ident()).collect::<Vec<_>>();
    let keys = fields.iter().map(|f| f.camel_lit()).collect::<Vec<_>>();
    let rty = fields.iter().map(|f| f.rty()).collect::<Vec<_>>();
    let fields = fields.iter().map(|f| f.ident).collect::<Vec<_>>();

    quote! {
        #[automatically_derived]
        #vis struct #reactive(#[doc(hidden)]vue::Reactive<#props>);

        #[automatically_derived]
        impl #reactive {
            pub fn map<F, R>(&mut self, map: F) -> R
            where
                F: FnOnce(&#props) -> R
            {
                self.0.map(map)
            }

            #[doc(hidden)]
            pub fn macro_set_active(&mut self, value: bool) {
                self.0.macro_set_active(value);
            }

            #[doc(hidden)]
            pub fn macro_set_target(&mut self, target: wasm_bindgen::JsValue) {
                self.0.macro_set_target(target);
            }

            #(
                pub fn #setters(&mut self, value: #types) {
                    self.0.macro_set(|p| &mut p.#fields, value, #keys, #rty);
                }
            )*
        }
    }
}

use crate::Input;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate(input: &Input) -> TokenStream {
    let fields = &input.fields;
    let ident = input.ident;
    let js_class = input.js_class_ident();
    let js_name = input.js_class_name_lit();
    let setters = fields.iter().map(|f| f.setter_ident()).collect::<Vec<_>>();
    let types = fields.iter().map(|f| f.ty).collect::<Vec<_>>();
    let fields = fields.iter().map(|f| f.ident).collect::<Vec<_>>();
    let reactive = input.reactive_ident();
    let update_fn = input.update_fn_ident();

    quote! {
        #[automatically_derived]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name)]
        pub struct #js_class(#[doc(hidden)]#reactive, #[doc(hidden)]vue::macros_utils::ReactiveDrop<#ident>);

        #[automatically_derived]
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #js_class {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                let reactive = vue::Reactive::<#ident>::new();
                vue::macros_utils::reactive_loop(|r| #update_fn(#reactive(r)), reactive.clone());
                Self(#reactive(reactive.clone()), vue::macros_utils::ReactiveDrop(reactive))
            }

            #[wasm_bindgen(setter)]
            pub fn set_component_active(&mut self, value: bool) {
                self.0.macro_set_active(value);
            }

            #[wasm_bindgen(setter)]
            pub fn set_data(&mut self, data: wasm_bindgen::JsValue) {
                self.0.macro_set_target(data);
            }

            #(
                #[wasm_bindgen(getter)]
                pub fn #fields(&mut self) -> #types {
                    self.0.map(|p| p.#fields.clone())
                }

                #[wasm_bindgen(setter)]
                pub fn #setters(&mut self, value: #types) {
                    self.0.#setters(value)
                }
            )*
        }
    }
}

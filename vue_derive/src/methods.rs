use heck::{ToLowerCamelCase, ToSnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{FnArg, Ident, ImplItem, ImplItemMethod, Item, LitInt, LitStr, Type, Visibility};

pub(crate) fn generate(input: &Item) -> TokenStream {
    let imp = match input {
        Item::Impl(imp) if imp.trait_.is_none() => imp,
        _ => panic!("Only struct impl are supported"),
    };

    let ident = match &*imp.self_ty {
        Type::Path(p) => p
            .path
            .get_ident()
            .expect("Only in scope type are supported"),
        _ => panic!("not supported"),
    };

    let component_name = component_name(&ident);
    let js_class = Ident::new(&format!("Js{component_name}"), ident.span());

    let methods = imp
        .items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Method(m) => Some(m),
            _ => None,
        })
        .filter(|m| match m.vis {
            Visibility::Public(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();

    let method_describes = methods_describe(&methods, &component_name);
    let methods = methods.iter().copied().map(method);

    quote! {
        #input

        #[automatically_derived]
        #[wasm_bindgen::prelude::wasm_bindgen]
        impl #js_class {
            #(#methods)*
        }

        #method_describes
    }
}

fn component_name(ident: &Ident) -> String {
    //Reactive$component_name

    const REACTIVE: &str = "Reactive";
    let s = ident.to_string();

    if s.len() <= REACTIVE.len() || !s.starts_with(REACTIVE) {
        panic!("This macro can only be used on the Reactive component.")
    }

    s[REACTIVE.len()..].to_string()
}

fn method(m: &ImplItemMethod) -> TokenStream {
    let sig = &m.sig;
    let ident = &sig.ident;

    let params = sig.inputs.iter().filter_map(|a| match a {
        FnArg::Receiver(_) => None,
        FnArg::Typed(t) => Some(&t.pat),
    });

    quote! {
        #[wasm_bindgen::prelude::wasm_bindgen(method)]
        pub #sig {
            self.#ident(#(#params),*)
        }
    }
}

fn method_describe(m: &ImplItemMethod) -> TokenStream {
    let name = m.sig.ident.to_string().to_lower_camel_case();
    let name = LitStr::new(&name, Span::call_site());

    let count = LitInt::new(
        &m.sig
            .inputs
            .iter()
            .filter(|a| match a {
                FnArg::Receiver(_) => false,
                FnArg::Typed(_) => true,
            })
            .count()
            .to_string(),
        Span::call_site(),
    );

    quote! { (#name, #count) }
}

fn methods_describe(methods: &[&ImplItemMethod], component_name: &str) -> TokenStream {
    let ident = component_name.to_snake_case();
    let ident = Ident::new(&format!("js_{ident}_methods"), Span::call_site());

    let js_name = component_name.to_lower_camel_case();
    let js_name = LitStr::new(&format!("{js_name}Methods"), Span::call_site());

    let methods = methods.iter().copied().map(method_describe);

    quote! {
        #[automatically_derived]
        #[doc(hidden)]
        #[wasm_bindgen::prelude::wasm_bindgen(js_name = #js_name)]
        pub fn #ident() -> wasm_bindgen::JsValue {
            wasm_bindgen::JsValue::from_serde(&[#(#methods),*]).unwrap()
        }
    }
}

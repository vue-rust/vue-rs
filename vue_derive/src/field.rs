use crate::FieldAttrs;
use darling::FromField;
use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Type};

pub(crate) struct Field<'a> {
    pub attrs: FieldAttrs,
    pub ident: &'a Ident,
    pub ty: &'a Type,
}

impl<'a> Field<'a> {
    pub fn from(f: &'a syn::Field) -> Option<Self> {
        Some(Self {
            ident: f.ident.as_ref()?,
            attrs: FieldAttrs::from_field(f).expect("attrs"),
            ty: &f.ty,
        })
    }

    pub fn camel_lit(&self) -> LitStr {
        LitStr::new(&self.camel_str(), self.ident.span())
    }

    pub fn camel_str(&self) -> String {
        self.ident.to_string().to_lower_camel_case()
    }

    pub fn rty(&self) -> TokenStream {
        if self.attrs.sync {
            quote! { vue::macros_utils::ReactiveTy::Sync }
        } else if self.attrs.prop {
            quote! { vue::macros_utils::ReactiveTy::Prop }
        } else {
            quote! { vue::macros_utils::ReactiveTy::Data }
        }
    }

    pub fn setter_ident(&self) -> Ident {
        let ident = self.ident;
        Ident::new(&format!("set_{ident}"), ident.span())
    }
}

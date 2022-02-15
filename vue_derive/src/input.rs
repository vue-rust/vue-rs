use crate::Field;
use heck::{ToLowerCamelCase, ToSnakeCase};
use syn::{Data, DeriveInput, Ident, LitStr, Visibility};

pub(crate) struct Input<'a> {
    pub fields: Vec<Field<'a>>,
    pub ident: &'a Ident,
    pub vis: &'a Visibility,
}

impl<'a> Input<'a> {
    pub fn from(input: &'a DeriveInput) -> Option<Self> {
        let fields = match &input.data {
            Data::Struct(s) => s.fields.iter().filter_map(Field::from).collect::<Vec<_>>(),
            _ => return None,
        };

        Some(Self {
            fields,
            ident: &input.ident,
            vis: &input.vis,
        })
    }

    pub fn js_class_ident(&self) -> Ident {
        let ident = self.ident;
        Ident::new(&format!("Js{ident}"), self.ident.span())
    }

    pub fn js_class_name_lit(&self) -> LitStr {
        LitStr::new(&self.ident.to_string(), self.ident.span())
    }

    pub fn js_props_ident(&self) -> Ident {
        let ident = self.ident.to_string().to_snake_case();
        Ident::new(&format!("js_{ident}_props"), self.ident.span())
    }

    pub fn js_props_lit(&self) -> LitStr {
        let ident = self.ident.to_string().to_lower_camel_case();
        LitStr::new(&format!("{ident}Props"), self.ident.span())
    }

    pub fn reactive_ident(&self) -> Ident {
        let ident = &self.ident;
        Ident::new(&format!("Reactive{ident}"), ident.span())
    }

    pub fn update_fn_ident(&self) -> Ident {
        let ident = self.ident.to_string().to_snake_case();
        Ident::new(&format!("{ident}_update"), self.ident.span())
    }
}

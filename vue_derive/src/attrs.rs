use darling::FromField;

#[derive(Debug, FromField)]
#[darling(attributes(vue))]
pub(super) struct FieldAttrs {
    #[darling(default)]
    pub prop: bool,

    #[darling(default)]
    pub sync: bool,
}

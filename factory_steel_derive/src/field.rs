use syn;
use meta;

pub struct Field {
    pub ty: syn::Type,
    pub name: syn::Ident,
    pub attrs: meta::Field
}

pub enum Def {
    StringDef(String),
    U8Def(u8)
}

impl Field {
    pub fn from_struct_field(field: &syn::Field) -> Self {
        let name = match field.ident {
            Some(f) => f,
            None => panic!("Unnamed fields not supported"),
        };
        let attrs = meta::Field::from_ast(&field.attrs);
        Self {
            ty: field.ty.clone(),
            name,
            attrs,
        }
    }
}
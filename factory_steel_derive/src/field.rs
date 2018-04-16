use syn;

pub struct Field {
    pub ty: syn::Type,
    pub name: syn::Ident,
}

impl Field {
    pub fn from_struct_field(field: &syn::Field) -> Self {
        let name = match field.ident {
            Some(f) => f,
            None => panic!("Unnamed fields not supported"),
        };
        Self {
            ty: field.ty.clone(),
            name,
        }
    }
}
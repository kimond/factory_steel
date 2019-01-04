use syn;
use quote::quote;
use quote::ToTokens;
use crate::meta;
use crate::lit;

pub struct Field {
    pub ty: syn::Type,
    pub ty_str: String,
    pub name: syn::Ident,
    pub attrs: meta::Field,
}

static INT_TYPES: [&'static str; 10] = [
    "usize", "u8", "u16", "u32", "u64",
    "isize", "i8", "i16", "i32", "i64"
];

static FLOAT_TYPES: [&'static str; 2] = [
    "f32", "f64"
];

impl Field {
    pub fn from_struct_field(field: &syn::Field) -> Self {
        let name = match &field.ident {
            Some(f) => f.to_owned(),
            None => panic!("Unnamed fields not supported"),
        };
        let attrs = meta::Field::from_ast(&field.attrs);
        let ty_str = type_as_str(field.ty.clone());
        Self {
            ty: field.ty.clone(),
            ty_str,
            name,
            attrs,
        }
    }

    pub fn get_default(&self) -> proc_macro2::TokenStream {
        let name = &self.name;
        let ty = &self.ty;
        let default = &self.attrs.default.clone().unwrap();
        if &self.ty_str == "String" {
            let value = lit::lit_to_string(default).unwrap();
            quote!(#name: #value.to_string())
        } else if &self.ty_str == "bool" {
            let value = lit::lit_to_bool(default).unwrap();
            quote!(#name: #value)
        } else if INT_TYPES.contains(&self.ty_str.as_ref()) {
            let value = lit::lit_to_int(default).unwrap();
            quote!(#name: #value as #ty)
        } else if FLOAT_TYPES.contains(&self.ty_str.as_ref()) {
            let value = lit::lit_to_float(default).unwrap();
            quote!(#name: #value as #ty)
        } else {
            panic!("Type {} not supported for field {}", &self.ty_str, &self.name.to_string(), )
        }
    }
}


fn type_as_str(ty: syn::Type) -> String {
    let field_type = match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            let mut tokens = proc_macro2::TokenStream::new();
            path.to_tokens(&mut tokens);
            tokens.to_string().replace(' ', "")
        }
        _ => panic!("Type not supported")
    };

    field_type
}


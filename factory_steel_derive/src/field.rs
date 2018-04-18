use syn;
use quote;
use quote::ToTokens;
use meta;
use lit;
use quote::Tokens;

pub struct Field {
    pub ty: syn::Type,
    pub ty_str: String,
    pub name: syn::Ident,
    pub attrs: meta::Field,
}

impl Field {
    pub fn from_struct_field(field: &syn::Field) -> Self {
        let name = match field.ident {
            Some(f) => f,
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

    pub fn get_default(&self) -> Tokens {
        let name = &self.name;
        let ty = &self.ty;
        let default = &self.attrs.default.clone().unwrap();
        match self.ty_str.as_ref() {
            "String" => {
                let value = lit::lit_to_string(default).unwrap();
                quote!(#name: #value.to_string())
            }
            "u8" | "u32" => {
                let value = lit::lit_to_int(default).unwrap();
                quote!(#name: #value as #ty)
            }
            _ => panic!("Type not supported {}", &self.ty_str)
        }
    }
}


fn type_as_str(ty: syn::Type) -> String {
    let field_type = match ty {
        syn::Type::Path(syn::TypePath { ref path, .. }) => {
            let mut tokens = quote::Tokens::new();
            path.to_tokens(&mut tokens);
            tokens.to_string().replace(' ', "")
        }
        _ => panic!("Type not supported")
    };

    field_type
}


#![recursion_limit = "512"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod field;
mod model;
mod meta;

use model::Model;
use syn::Lit::{Str, Int};

#[proc_macro_derive(Factory, attributes(facto))]
pub fn derive_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();

    let gen = impl_factory(&input);

    gen.into()
}

fn impl_factory(item: &syn::DeriveInput) -> quote::Tokens {
    let model = Model::from_item(&item);
    let struct_name = item.ident;
    let fields = model.fields.iter().map(|f| {
        let name = f.name;
        let ty = &f.ty;
        if let Some(ref value) = f.attrs.default {
            match *value {
                Str(ref s) => quote!(#name: #value.to_string()),
                Int(ref s) => quote!(#name: #value),
                _ => quote!(#name: #ty::default())
            }
        } else {
            quote!(#name: #ty::default())
        }
    });
    quote! {
        impl Factory for #struct_name {
            fn create() -> Self {
                Self{
                    #(#fields),*
                }
            }
        }
    }
}

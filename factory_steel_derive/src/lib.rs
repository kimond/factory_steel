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
mod lit;

use model::Model;

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
        if f.attrs.default.is_some() {
            f.get_default()
        } else if f.attrs.is_fake {
            let fake_value = f.attrs.fake_value.clone().unwrap();
            quote!(#name: fake!(#fake_value))
        } else if f.attrs.is_sub_factory {
            quote!(#name: #ty::create())
        } else {
            quote!(#name: #ty::default())
        }
    });
    let has_fake_fields = model.fields.iter().any(|f| f.attrs.is_fake);
    let fake_import = match has_fake_fields {
        true => quote!(use factory_steel::fake::*;),
        false => quote!()
    };
    quote! {
        #fake_import

        impl Factory for #struct_name {
            fn create() -> Self {
                Self{
                    #(#fields),*
                }
            }
        }
    }
}

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Factory)]
pub fn derive_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();

    let gen = impl_factory(&input);

    println!("{}", gen.to_string());
    gen.into()
}

fn impl_factory(item: &syn::DeriveInput) -> quote::Tokens {
    let fields: syn::Fields;
    let name = &item.ident;
    match &item.data {
        &syn::Data::Struct(ref data) => {
            fields = data.fields.clone();
        }
        _ => {
            panic!("This macro can be used with Struct only!")
        }
    }
    let field_names = fields.iter().map(|f| f.ident.unwrap());
    quote! {
        impl Factory for #name {
            fn create() -> Self {
                Self{
                    #(#field_names: "".to_string()),*
                }
            }
        }
    }
}

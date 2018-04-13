extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Factory)]
pub fn derive_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let gen = impl_factory(&ast);

    gen.into()
}

fn impl_factory(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Factory for #name {
            fn create() -> Self {
                Self{}
            }
        }
    }
}

#[proc_macro_derive(FieldInfo)]
pub fn derive_field_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    let gen = impl_field_info(&input);

    gen.into()
}

fn impl_field_info(item: &syn::DeriveInput) -> quote::Tokens {
    let name = &item.ident;
    quote! {
        impl #name {
            fn fields_info() {
                println!("field")
            }
        }
    }
}

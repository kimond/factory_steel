extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[proc_macro_derive(Factory)]
pub fn derive_factory(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let s = input.to_string();
    println!("{}",s);

    let ast = syn::parse_str(&s).unwrap();

    let gen = impl_factory(&ast);

    gen.into()
}

fn impl_factory(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Factory for #name {
            fn create() {
                println!("hello");
            }
        }
    }
}

#[proc_macro_derive(FieldInfo)]
pub fn derive_field_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    input
}

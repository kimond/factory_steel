extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;


trait Factory {
    fn create();
}

#[proc_macro_derive(Factory)]
pub fn factory_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
//        #[derive(Factory)]
//        struct MyModel {
//
//        }
//
//        let m = MyModel::create{};
    }
}

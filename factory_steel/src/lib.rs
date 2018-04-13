#[macro_use]
extern crate factory_steel_derive;
pub use factory_steel_derive::*;

#[macro_use]
mod macros;

pub trait Factory {
    fn create() -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_faking() {
        struct Person {
            name: String
        }

        impl Factory for Person {
            fn create() -> Self {
                return Person { name: "hola".to_string() };
            }
        }

        Person::create();
    }
}
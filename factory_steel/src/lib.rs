#[macro_use]
extern crate factory_steel_derive;
pub use factory_steel_derive::*;

macro_rules! fields_information {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        struct $name {
            $($fname : $ftype),*
        }

        println!("{}", $ftype)
        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}

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
#[macro_use]
extern crate factory_steel_derive;

pub use factory_steel_derive::*;

pub trait Factory {
    fn create() -> Self;
}

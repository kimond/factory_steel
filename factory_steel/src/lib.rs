//! # Factory_steel
//! Factory_steel is a fixture replacement greatly inspired by [factory_bot](https://github.com/thoughtbot/factory_bot)
//! and [factory_boy](https://github.com/FactoryBoy/factory_boy). Currently it is an experimental project.

#[allow(unused_imports)]
#[macro_use]
extern crate factory_steel_derive;

#[doc(hidden)]
pub use factory_steel_derive::*;


/// A generic factory
///
/// This trait is automatically implemented using the `factory` derive.
/// It is also possible to implement it manually if you want to define custom behaviors for
/// nested factories.
pub trait Factory {
    fn create() -> Self;
}

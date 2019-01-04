//! # Factory_steel
//! Factory_steel is a fixture replacement greatly inspired by [factory_bot](https://github.com/thoughtbot/factory_bot)
//! and [factory_boy](https://github.com/FactoryBoy/factory_boy). Currently it is an experimental project.

use ::fake as fake_rs;
pub mod fake {
    pub use crate::fake_rs::*;
}

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

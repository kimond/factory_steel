# Factory_Steel
[![Build Status](https://travis-ci.org/kimond/factory_steel.svg?branch=master)](https://travis-ci.org/kimond/factory_steel)

Factory_steel is a fixture replacement greatly inspired by [factory_bot](https://github.com/thoughtbot/factory_bot)
and [factory_boy](https://github.com/FactoryBoy/factory_boy). Currently it is an experimental project. 
Although usable, it is still very limited in term of features and reliability.

## Getting started
Add this to your `Cargo.toml`:
```toml
[dependencies]
factory_steel = "0.1.0"
```

## Example

```rust
#[macro_use]
extern crate factory_steel;
use factory_steel::Factory;

#[derive(Factory)]
struct Post {
    #[facto(default="New post")]
    title: String,
}

fn main() {
    let post = Post::create();
    
    //Prints New post
    println!("{}", post.title);
}
```

## Contributing
Contributions are welcome! This crate is work in progress, and a lot of work remains to be done.
Have a look at the [issues](https://github.com/kimond/factory_steel/issues), and 
open a pull request if you'd like to add some functionality.

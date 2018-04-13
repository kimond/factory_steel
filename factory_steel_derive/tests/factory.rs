#[macro_use]
extern crate factory_steel;

use factory_steel::Factory;

#[test]
fn it_works() {
    #[derive(Factory)]
    struct MyModel {

    }

    let m = MyModel::create();
}

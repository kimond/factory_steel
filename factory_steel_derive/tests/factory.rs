use factory_steel::Factory;

#[test]
fn derive_factory_with_simple_struct() {
    #[derive(Factory)]
    struct MyModel {

    }

    MyModel::create();
}

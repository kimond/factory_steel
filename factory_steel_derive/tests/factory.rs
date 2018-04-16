use factory_steel::Factory;

#[test]
fn derive_factory_with_only_default() {
    #[derive(Factory)]
    struct MyModel {
        name: String,
        age: u8,
        height: f32,
    }

    let m = MyModel::create();
    assert_eq!(m.name, String::default());
    assert_eq!(m.age, u8::default());
    assert_eq!(m.height, f32::default());
}

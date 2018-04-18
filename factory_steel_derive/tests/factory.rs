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

#[test]
fn derive_factory_with_field_default_option() {
    #[derive(Factory)]
    struct MyModel {
        #[facto(default="John")]
        name: String,
        #[facto(default=23)]
        age: u8,
    }

    let m = MyModel::create();
    assert_eq!(m.name, "John");
    assert_eq!(m.age, u8::default());
}

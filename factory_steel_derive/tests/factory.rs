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
        #[facto(default = "John")]
        name: String,
        #[facto(default = "23")]
        age: u8,
        #[facto(default = "44")]
        distance: u32,
        #[facto(default = "33")]
        axis: i64,
        #[facto(default = "3.3")]
        speed: f64,
        #[facto(default = "true")]
        is_ok: bool,
    }

    let m = MyModel::create();
    assert_eq!(m.name, "John");
    assert_eq!(m.age, 23);
    assert_eq!(m.distance, 44);
    assert_eq!(m.axis, 33);
    assert_eq!(m.speed, 3.3);
    assert_eq!(m.is_ok, true);
}

#[test]
fn derive_factory_with_sub_factory() {
    #[derive(Factory)]
    struct SubModel {
        name: String
    }

    #[derive(Factory)]
    struct MyModel {
        name: String,
        #[facto(sub_factory)]
        sub_model: SubModel
    }

    let m = MyModel::create();
    assert_eq!(m.name, String::default());
    assert_eq!(m.sub_model.name, String::default());
}


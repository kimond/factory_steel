use factory_steel::Factory;

#[test]
fn derive_factory_with_string_fields_only() {
    #[derive(Factory)]
    struct MyModel {
        name: String,
        job: String
    }

    let m = MyModel::create();
    assert_eq!(m.name, "".to_string());
    assert_eq!(m.job, "".to_string());
}

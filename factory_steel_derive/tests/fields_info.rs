#[test]
fn derive_field_info_on_struct() {
    #[derive(FieldInfo)]
    struct MyModel {
        name: String,
        age: u32
    }

    let m = MyModel::fields_info();
}

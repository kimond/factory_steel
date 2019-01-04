use syn;
use crate::field::Field;

pub struct Model {
    pub name: syn::Ident,
    pub fields: Vec<Field>,
}

impl Model {
    pub fn from_item(item: &syn::DeriveInput) -> Self {
        let fields = fields_from_item_data(&item.data).unwrap();
        Self {
            name: item.ident.to_owned(),
            fields,
        }
    }
}

fn fields_from_item_data(data: &syn::Data) -> Result<Vec<Field>, &str> {
    use syn::Data::*;

    let struct_data = match *data {
        Struct(ref d) => d,
        _ => return Err("This derive can only be used on structs"),
    };

    Ok(struct_data.fields
        .iter()
        .map(|f| Field::from_struct_field(f))
        .collect()
    )
}


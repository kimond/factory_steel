use syn;
use syn::Meta::{List, NameValue, Word};
use syn::NestedMeta::{Literal, Meta};

const ATTR_NAME: &'static str = "facto";

pub struct Field {
    pub default: Option<syn::Lit>,
    pub is_sub_factory: bool,
    pub is_fake: bool,
    pub fake_value: Option<syn::Lit>,
}

impl Field {
    pub fn from_ast(attrs: &[syn::Attribute]) -> Self {
        let mut is_sub_factory = false;
        let mut default: Option<syn::Lit> = None;
        let mut is_fake = false;
        let mut fake_value = None;
        for meta_items in attrs.iter().filter_map(get_facto_meta_items) {
            for meta_item in meta_items {
                match meta_item {
                    Meta(NameValue(ref m)) if m.ident == "default" => {
                        default = Some(m.lit.clone());
                    }
                    Meta(Word(ref word)) if word == "sub_factory" => {
                        is_sub_factory = true;
                    }
                    Meta(NameValue(ref m)) if m.ident == "fake" => {
                        is_fake = true;
                        fake_value = Some(m.lit.clone());
                    }
                    Meta(ref meta_item) => {
                        panic!("Unknown factory_steel attribute {}", meta_item.name());
                    }
                    Literal(_) => {
                        panic!("Unexpected literal in factory_steel attribute");
                    }
                }
            }
        }
        Field {
            default,
            is_sub_factory,
            is_fake,
            fake_value,
        }
    }
}


pub fn get_facto_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1 && attr.path.segments[0].ident == ATTR_NAME {
        match attr.interpret_meta() {
            Some(List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            _ => None
        }
    } else {
        None
    }
}
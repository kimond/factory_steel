use syn;


pub fn lit_to_string(lit: &syn::Lit) -> Option<String> {
    match *lit {
        syn::Lit::Str(ref s) => Some(s.value()),
        _ => None,
    }
}

pub fn lit_to_int(lit: &syn::Lit) -> Option<u64> {
    match *lit {
        syn::Lit::Int(ref s) => Some(s.value()),
        syn::Lit::Str(ref s) => Some(s.value().parse::<u64>().unwrap()),
        _ => None,
    }
}
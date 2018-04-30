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

pub fn lit_to_float(lit: &syn::Lit) -> Option<f64> {
    match *lit {
        syn::Lit::Float(ref s) => Some(s.value()),
        syn::Lit::Int(ref s) => Some(s.value() as f64),
        syn::Lit::Str(ref s) => Some(s.value().parse::<f64>().unwrap()),
        _ => None,
    }
}

pub fn lit_to_bool(lit: &syn::Lit) -> Option<bool> {
    match *lit {
        syn::Lit::Bool(ref s) => Some(s.value),
        syn::Lit::Str(ref s) => if s.value() == "true" { Some(true) } else { Some(false) },
        _ => None,
    }
}

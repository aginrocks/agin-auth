use quote::format_ident;

pub fn associated_type(base: &syn::TypePath, trait_path: &str, member: &str) -> syn::TypePath {
    let trait_path = syn::parse_str::<syn::Path>(trait_path).unwrap();
    let member_ident = format_ident!("{member}");

    let tokens = quote::quote! {
        <#base as #trait_path>::#member_ident
    };

    syn::parse2(tokens).expect("valid associated type path")
}

pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

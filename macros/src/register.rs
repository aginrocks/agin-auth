use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::register::{entry::FactorEntry, list::FactorList};

pub mod entry;
pub mod list;

pub fn register(factor_list: FactorList) -> Result<proc_macro::TokenStream, syn::Error> {
    let mut output = TokenStream::new();

    let mut seen_factors = HashSet::new();

    let mut router = quote! { ::utoipa_axum::router::OpenApiRouter::new() };
    let mut enum_variants = TokenStream::new();

    for FactorEntry { path, slug } in &factor_list.entries {
        // Ensure factor is unique
        if seen_factors.contains(slug) {
            return Err(syn::Error::new_spanned(
                slug,
                "Each factor has to be unique",
            ));
        }
        seen_factors.insert(slug);

        let last_segment = path
            .segments
            .last()
            .ok_or_else(|| syn::Error::new_spanned(path, "expected a non-empty path"))?;

        let module_segment = path
            .segments
            .iter()
            .nth_back(1)
            .ok_or_else(|| syn::Error::new_spanned(path, "expected a module path"))?;

        let last_segment_str =
            LitStr::new(&last_segment.ident.to_string(), last_segment.ident.span());

        // Validate the slug
        let slug_assertion = quote! {
            const _: () = assert!(
                ::auth_core::str_eq(<#path as ::auth_core::Factor>::SLUG, #slug),
                concat!("slug mismatch for factor `", #last_segment_str, "`: slug `", #slug, "` doesn't match trait definition")
            );
        };
        output.extend(slug_assertion);

        // Register the factor to the router
        router.extend(quote! {
            .merge(#module_segment::routes())
        });

        // Register the factor to the enum
        let variant_name = &last_segment.ident;
        enum_variants.extend(quote! {
            #variant_name(<#path as ::auth_core::Factor>::Config),
        });
    }

    let handler = quote! {
        pub fn routes() -> OpenApiRouter<AppState> {
            #router
        }
    };
    output.extend(handler);

    let factors_enum = quote! {
        pub enum FactorConfig {
            #enum_variants
        }
    };
    output.extend(factors_enum);

    Ok(output.into())
}

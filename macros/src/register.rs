use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::register::{
    entry::FactorEntry,
    enums::{factor_config, factor_name},
    list::FactorList,
};

pub mod entry;
pub mod enums;
pub mod list;

pub fn register(factor_list: &FactorList) -> Result<proc_macro::TokenStream, syn::Error> {
    let mut output = TokenStream::new();

    let mut seen_factors = HashSet::new();

    let mut router = quote! { ::utoipa_axum::router::OpenApiRouter::new() };

    for FactorEntry {
        path,
        slug,
        last_segment,
        module_segment,
    } in &factor_list.entries
    {
        // Ensure factor is unique
        if seen_factors.contains(slug) {
            return Err(syn::Error::new_spanned(
                slug,
                "Each factor has to be unique",
            ));
        }
        seen_factors.insert(slug);

        let last_segment_str =
            LitStr::new(&last_segment.ident.to_string(), last_segment.ident.span());

        // Validate the slug
        let slug_assertion = quote! {
            #[doc(hidden)]
            const _: () = assert!(
                ::auth_core::str_eq(<#path as ::auth_core::FactorSlug>::SLUG, #slug),
                concat!("slug missmatch for factor `", #last_segment_str, "`: slug `", #slug, "` doesn't match trait definition")
            );
        };
        output.extend(slug_assertion);

        // Register the factor to the router
        router.extend(quote! {
            .merge(#module_segment::routes())
        });
    }

    let handler = quote! {
        pub fn routes() -> ::utoipa_axum::router::OpenApiRouter<crate::state::AppState> {
            #router
        }
    };
    output.extend(handler);

    output.extend(factor_config(factor_list));
    output.extend(factor_name(factor_list));

    Ok(output.into())
}

use proc_macro2::TokenStream;
use quote::quote;

use crate::{FactorList, register::entry::FactorEntry};

/// `FactorConfig` enum with all possible configs
pub fn factor_config(factor_list: &FactorList) -> TokenStream {
    let mut enum_variants = TokenStream::new();

    for FactorEntry {
        path,
        slug: _,
        last_segment,
        module_segment: _,
    } in &factor_list.entries
    {
        // Register the factor to the enum
        let variant_name = &last_segment.ident;
        enum_variants.extend(quote! {
            #variant_name(<#path as ::auth_core::Factor>::Config),
        });
    }

    quote! {
        pub enum FactorConfig {
            #enum_variants
        }
    }
}

pub fn factor_name(factor_list: &FactorList) -> TokenStream {
    let mut enum_variants = TokenStream::new();

    for FactorEntry {
        path: _,
        slug,
        last_segment,
        module_segment: _,
    } in &factor_list.entries
    {
        // Register the factor to the enum
        let variant_name = &last_segment.ident;
        enum_variants.extend(quote! {
            #[serde(rename = #slug)]
            #variant_name,
        });
    }

    // TODO: Add `ToFacotrName` trait and autoamtically implement it for factors
    quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub enum FactorName {
            #enum_variants
        }
    }
}

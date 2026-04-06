mod definitions;
mod utils;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::{
    FactorList,
    register::{
        entry::FactorEntry,
        enums::{
            definitions::{FLOW_TYPE, ROLE, SECURITY_LEVEL},
            utils::implement_field_match,
        },
    },
};

/// `FactorConfig` enum with all possible configs
pub fn factor_config(factor_list: &FactorList) -> TokenStream {
    let mut enum_variants = TokenStream::new();

    for FactorEntry {
        path,
        slug,
        last_segment,
        module_segment: _,
    } in &factor_list.entries
    {
        // Register the factor to the enum
        let variant_name = &last_segment.ident;
        enum_variants.extend(quote! {
            #[serde(rename = #slug)]
            #variant_name(<#path as ::auth_core::Factor>::Config),
        });
    }

    quote! {
        #[derive(Clone, Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub enum FactorConfig {
            #enum_variants
        }
    }
}

pub fn factor_name(factor_list: &FactorList) -> TokenStream {
    let mut enum_variants = TokenStream::new();
    let mut output = TokenStream::new();

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

    let factor_name_ty: Type = syn::parse_str("FactorName").expect("valid type");

    // TODO: Add `ToFactorName` trait and autoamtically implement it for factors
    output.extend(quote! {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub enum #factor_name_ty {
            #enum_variants
        }
    });

    output.extend(implement_field_match(
        factor_list,
        &[&FLOW_TYPE, &SECURITY_LEVEL, &ROLE],
        &factor_name_ty,
        &syn::parse_str("::auth_core::FactorMetadataDynamic").expect("valid path"),
    ));

    output
}

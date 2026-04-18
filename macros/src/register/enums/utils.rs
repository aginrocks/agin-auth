use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Path, Type};

use crate::{FactorList, register::enums::definitions::FieldDefinition};

pub fn match_factors(factor_list: &FactorList, field_definition: &FieldDefinition) -> TokenStream {
    let trait_path: Path =
        syn::parse_str(field_definition.trait_path).expect("hardcoded path should parse");
    let target_field = format_ident!("{}", field_definition.field_name);

    let arms = factor_list.entries.iter().map(|factor| {
        let variant_name = &factor.last_segment.ident;
        let path = &factor.path;

        quote! {
            Self::#variant_name => <#path as #trait_path>::#target_field
        }
    });

    quote! {
        #(#arms),*
    }
}

pub fn implement_field_match(
    factor_list: &FactorList,
    fields: &[&FieldDefinition],
    self_ty: &Type,
    trait_ty: &Type,
) -> TokenStream {
    let methods = fields
        .iter()
        .map(|field| {
            let method_name = format_ident!("{}", field.method_name);
            let method_type: Type =
                syn::parse_str(field.field_type).expect("hardcoded types should parse");
            let match_arms = match_factors(factor_list, field);

            quote! {
                fn #method_name(&self) -> #method_type {
                    match self {
                        #match_arms
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        impl #trait_ty for #self_ty {
            #(#methods)*
        }
    }
}

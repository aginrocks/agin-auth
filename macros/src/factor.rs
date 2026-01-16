use quote::ToTokens;

use crate::factor::{definitions::AUTHENTICATE, generate_handler::generate_handler};

pub mod args;
pub mod definitions;
mod generate_handler;

pub fn factor(
    args: args::FactorArgs,
    input: syn::ItemImpl,
) -> Result<proc_macro::TokenStream, darling::Error> {
    let self_ty = match &*input.self_ty {
        syn::Type::Path(p) => p.clone(),
        _ => {
            return Err(darling::Error::custom(
                "Expected a type path for the impl block's self type",
            ));
        }
    };

    let args = generate_handler::HandlerUserArgs {
        base_struct: self_ty,
        doc: None,
        factor_name: args.name,
        factor_slug: args.slug,
    };

    let handler = generate_handler(&AUTHENTICATE, args);

    let original_impl = input.into_token_stream();

    let output = quote::quote! {
        #original_impl
        #handler
    };

    Ok(output.into())
}

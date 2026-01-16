use quote::ToTokens;

use crate::{
    factor::{definitions::METHODS, generate_handler::generate_handler, router::generate_router},
    util::extract_methods,
};

pub mod args;
pub mod definitions;
mod generate_handler;
mod router;

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

    let methods = extract_methods(input.clone());
    let mut tokens = input.into_token_stream();
    let mut routes = Vec::new();

    for supported_method in METHODS {
        let Some(method_impl) = methods.get(supported_method.method) else {
            continue;
        };

        let doc = method_impl
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc"))
            .cloned()
            .collect();

        let args = generate_handler::HandlerUserArgs {
            base_struct: &self_ty,
            doc,
            factor_name: &args.name,
            factor_slug: &args.slug,
        };

        let (name, handler) = generate_handler(supported_method, args)?;
        routes.push(name);
        tokens.extend(handler);
    }

    let router = generate_router(routes);
    tokens.extend(router);

    Ok(tokens.into())
}

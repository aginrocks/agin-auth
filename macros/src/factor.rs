use quote::ToTokens;

use crate::{
    factor::{
        args::ImplKind, definitions::METHODS, generate_handler::generate_handler,
        router::generate_router,
    },
    util::{const_impl_exists, extract_methods},
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

    let trait_ty = match &input.trait_ {
        Some((_, path, _)) => path.clone(),
        None => {
            return Err(darling::Error::custom(
                "Expected the impl block to implement a trait",
            ));
        }
    };

    let mut input = input;

    let impl_kind = ImplKind::detect(&trait_ty)?;

    let slug = args.slug.as_str();
    let skip_slug = const_impl_exists(&input, syn::parse_quote!(SLUG));
    if !skip_slug && impl_kind == ImplKind::Factor {
        let slug_item: syn::ImplItem = syn::parse_quote! {
            const SLUG: &'static str = #slug;
        };
        input.items.insert(0, slug_item);
    }

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
            factor_slug: &args.slug,
            applied_trait: &trait_ty,
        };

        let (name, handler) = generate_handler(supported_method, args)?;
        routes.push(name);
        tokens.extend(handler);
    }

    let router = generate_router(routes, impl_kind);
    tokens.extend(router);

    // Validate the slug
    let slug_assertion = quote::quote! {
        const _: () = assert!(
            ::auth_core::str_eq(<#self_ty as ::auth_core::Factor>::SLUG, #slug),
            "slug missmatch"
        );
    };
    tokens.extend(slug_assertion);

    Ok(tokens.into())
}

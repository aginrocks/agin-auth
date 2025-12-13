mod factor;

use proc_macro::TokenStream;

/// Generate Axum handlers annotated with `utoipa` for the factor.
/// This allows for end-to-end typing of authentication factors in the API documentation.
#[proc_macro_attribute]
pub fn factor(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match darling::ast::NestedMeta::parse_meta_list(args.into()) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error().into(),
    };

    let args = match <factor::args::FactorArgs as darling::FromMeta>::from_list(&args) {
        Ok(x) => x,
        Err(e) => return e.write_errors().into(),
    };

    let input = syn::parse_macro_input!(input as syn::ItemStruct);

    match factor::factor(args, input) {
        Ok(x) => x,
        Err(e) => e.write_errors().into(),
    }
}

use inflector::Inflector;
use quote::format_ident;
use syn::Attribute;

use crate::{
    factor::definitions::{EndpointBasePath, HandlerDefinition},
    util::{associated_type, capitalize_first, wrap_with_generic},
};

// User-provided args for handler generation
pub struct HandlerUserArgs<'a> {
    /// Base struct type that the macro is applied to
    pub base_struct: &'a syn::TypePath,

    /// Name of the factor (e.g. Password)
    pub factor_name: &'a str,

    /// A slug that will appear in the URL (e.g. password)
    pub factor_slug: &'a str,

    /// Extracted doc comment for the handler
    pub doc: Vec<Attribute>,
}

pub fn generate_handler(
    definition: &HandlerDefinition,
    args: HandlerUserArgs,
) -> Result<(syn::Ident, proc_macro2::TokenStream), darling::Error> {
    let doc = args.doc;

    let request = associated_type(
        args.base_struct,
        "::auth_core::Factor",
        definition.request_type,
    )?;

    let success = associated_type(
        args.base_struct,
        "::auth_core::Factor",
        definition.response_type,
    )?;

    let success = match definition.response_generic {
        Some(generic) => wrap_with_generic(&success, generic)?,
        None => success,
    };

    let error = syn::parse_str::<syn::Path>(definition.error_type)?;

    let error_status = match definition.endpoint_base_path {
        EndpointBasePath::Authentication => quote::quote! { UNAUTHORIZED },
        EndpointBasePath::Management => quote::quote! { BAD_REQUEST },
    };

    let tag = match definition.endpoint_base_path {
        EndpointBasePath::Authentication => "Auth",
        EndpointBasePath::Management => "Settings",
    };

    let path = format!(
        "{}/{}/{}",
        definition.endpoint_base_path, args.factor_slug, definition.endpoint
    );

    let success_ident = format_ident!(
        "{}{}Response",
        capitalize_first(&definition.method.to_camel_case()),
        capitalize_first(&args.factor_slug.to_camel_case())
    );

    let request_ident = format_ident!(
        "{}{}Request",
        capitalize_first(&definition.method.to_camel_case()),
        capitalize_first(&args.factor_slug.to_camel_case())
    );

    let function_name = format_ident!(
        "{}_{}",
        args.factor_slug.to_snake_case(),
        definition.method.to_snake_case()
    );

    let tokens = quote::quote! {
        type #success_ident = #success;
        type #request_ident = #request;

        #(#doc)*
        #[::utoipa::path(
            method(post),
            path = #path,
            responses(
                (status = OK, description = "Success", body = #success_ident, content_type = "application/json"),
                (status = #error_status, description = "Error", body = #error, content_type = "application/json")
            ),
            tag = #tag
        )]
        pub async fn #function_name(
            ::axum::Extension(state): ::axum::Extension<crate::state::AppState>,
            session: ::tower_sessions::Session,
            ::axum::Json(body): ::axum::Json<#request_ident>,
        ) -> crate::axum_error::AxumResult<::axum::Json<#success>> {
            todo!()
        }
    };

    Ok((function_name, tokens))
}

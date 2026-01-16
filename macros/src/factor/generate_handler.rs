use inflector::Inflector;
use quote::format_ident;

use crate::{
    factor::definitions::{EndpointBasePath, HandlerDefinition},
    util::{associated_type, capitalize_first},
};

// User-provided args for handler generation
pub struct HandlerUserArgs {
    /// Base struct type that the macro is applied to
    pub base_struct: syn::TypePath,

    /// Name of the factor (e.g. Password)
    pub factor_name: String,

    /// A slug that will appear in the URL (e.g. password)
    pub factor_slug: String,

    /// Extracted doc comment for the handler
    pub doc: Option<String>,
}

pub fn generate_handler(
    definition: &HandlerDefinition,
    args: HandlerUserArgs,
) -> proc_macro2::TokenStream {
    let doc = if let Some(doc) = args.doc {
        quote::quote! {
            #[doc = #doc]
        }
    } else {
        quote::quote! {}
    };

    let success = associated_type(
        &args.base_struct,
        "::auth_core::Factor",
        definition.response_type,
    );

    let error = syn::parse_str::<syn::Path>(definition.error_type).unwrap();

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
        capitalize_first(&args.factor_slug.to_camel_case()),
        capitalize_first(&definition.method.to_camel_case())
    );

    quote::quote! {
        type #success_ident = #success;

        #doc
        #[::utoipa::path(
            method(post),
            path = #path,
            responses(
                (status = OK, description = "Success", body = #success_ident, content_type = "application/json"),
                (status = #error_status, description = "Error", body = #error, content_type = "application/json")
            ),
            tag = #tag
        )]
        pub async fn foo() -> String {
            String::from(#path)
        }
    }
}

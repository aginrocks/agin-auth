use quote::format_ident;

use crate::factor::definitions::{EndpointBasePath, HandlerDefinition};

// User-provided args for handler generation
pub struct HandlerUserArgs {
    /// Base struct type that the macro is applied to
    pub base_struct: syn::TypePath,

    /// Name of the factor (e.g. Password)
    // pub factor_name: String,

    /// A slug that will appear in the URL (e.g. password)
    // pub factor_slug: String,

    /// Extracted doc comment for the handler
    pub doc: Option<String>,
}

fn associated_type(base: &syn::TypePath, trait_path: &str, member: &str) -> syn::TypePath {
    let trait_path = syn::parse_str::<syn::Path>(trait_path).unwrap();
    let member_ident = format_ident!("{member}");

    let tokens = quote::quote! {
        <#base as #trait_path>::#member_ident
    };

    syn::parse2(tokens).expect("valid associated type path")
}

pub fn generate_handler(
    definition: &HandlerDefinition,
    args: HandlerUserArgs,
    i: u32,
) -> proc_macro2::TokenStream {
    let doc = if let Some(doc) = args.doc {
        quote::quote! {
            #[doc = #doc]
        }
    } else {
        quote::quote! {}
    };

    let base_path: &'static str = definition.endpoint_base_path.into();
    let endpoint_name = definition.endpoint;

    let slug = associated_type(&args.base_struct, "::auth_core::Factor", "SLUG");

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

    let success_ident = format_ident!("__Success_{i}");

    quote::quote! {
        type #success_ident = #success;

        #doc
        #[::utoipa::path(
            method(post),
            path = format!("{}/{}/{}", #base_path, #slug, #endpoint_name),
            responses(
                (status = OK, description = "Success", body = #success_ident, content_type = "application/json"),
                (status = #error_status, description = "Error", body = #error, content_type = "application/json")
            )
            tag = #tag
        )]
        async fn foo() -> String {
            String::from(format!("{}/{}/{}", #base_path, #slug, #endpoint_name))
        }
    }
}

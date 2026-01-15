use quote::format_ident;



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

fn associated_type(base: &syn::TypePath, trait_path: &str, member: &str) -> syn::TypePath {
    let trait_path = syn::parse_str::<syn::Path>(trait_path).unwrap();
    let member_ident = format_ident!("{member}");

    let tokens = quote::quote! {
        <#base as #trait_path>::#member_ident
    };

    syn::parse2(tokens).expect("valid associated type path")
}

pub fn generate_handler(
    definition: HandlerDefinition,
    args: HandlerUserArgs,
) -> proc_macro2::TokenStream {
    let doc = if let Some(doc) = args.doc {
        quote::quote! {
            #[doc = #doc]
        }
    } else {
        quote::quote! {}
    };

    let path = format!(
        "{}/{}/{}",
        definition.endpoint_base_path, args.factor_slug, definition.endpoint_name
    );

    let success = associated_type(
        &args.base_struct,
        "::auth_core::Factor",
        definition.response_type_name,
    );

    quote::quote! {
        #doc
        #[::utoipa::path(
            method(post),
            path = #path,
            responses(
                (status = OK, description = "Success", body = #success)
            )
        )]
        fn foo() {}
    }
}

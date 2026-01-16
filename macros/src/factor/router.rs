pub fn generate_router(routes: Vec<syn::Ident>) -> proc_macro2::TokenStream {
    let route_calls = routes.iter().map(|route| {
        quote::quote! {
            .routes(::utoipa_axum::routes!(#route))
        }
    });

    quote::quote! {
        pub fn routes() -> ::utoipa_axum::router::OpenApiRouter<crate::state::AppState> {
            ::utoipa_axum::router::OpenApiRouter::new()
                #(#route_calls)*
        }
    }
}

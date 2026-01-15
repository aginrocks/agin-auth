pub mod args;
pub mod definitions;
mod generate_handler;

pub fn factor(
    args: args::FactorArgs,
    input: syn::ItemImpl,
) -> Result<proc_macro::TokenStream, darling::Error> {
    todo!()
}

use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct FactorArgs {
    pub path: String,

    #[darling(default)]
    pub requires_confirmation: bool,
}

use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct FactorArgs {
    pub slug: String,
}

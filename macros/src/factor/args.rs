use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct FactorArgs {
    pub name: String,
    pub slug: String,
}

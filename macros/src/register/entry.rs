use syn::{
    Ident, LitStr, Path, Token,
    parse::{Parse, ParseStream},
};

pub struct FactorEntry {
    pub slug: LitStr,
    pub path: Path,
}

impl Parse for FactorEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let slug = input.parse::<LitStr>()?;
        input.parse::<Token![=>]>()?;
        let path = input.parse::<Path>()?;
        Ok(FactorEntry { slug, path })
    }
}

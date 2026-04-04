use syn::{
    LitStr, Path, PathSegment, Token,
    parse::{Parse, ParseStream},
};

pub struct FactorEntry {
    pub slug: LitStr,
    pub path: Path,
    pub last_segment: PathSegment,
    pub module_segment: PathSegment,
}

impl Parse for FactorEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let slug = input.parse::<LitStr>()?;
        input.parse::<Token![=>]>()?;

        let path = input.parse::<Path>()?;

        let last_segment =
            path.segments.last().cloned().ok_or_else(|| {
                syn::Error::new_spanned(path.clone(), "expected a non-empty path")
            })?;

        let module_segment = path
            .segments
            .iter()
            .nth_back(1)
            .cloned()
            .ok_or_else(|| syn::Error::new_spanned(path.clone(), "expected a module path"))?;

        Ok(FactorEntry {
            slug,
            path,
            last_segment,
            module_segment,
        })
    }
}

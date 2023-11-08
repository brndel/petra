use quote::ToTokens;


#[derive(Clone)]
pub enum ForeignRule {
  Cascade,
  SetNull,
  Restrict,
}

impl Default for ForeignRule {
  fn default() -> Self {
    Self::Cascade
  }
}

impl ToTokens for ForeignRule {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let q = match self {
      ForeignRule::Cascade => quote!(mensula::ForeignRule::Cascade),
      ForeignRule::SetNull => quote!(mensula::ForeignRule::SetNull),
      ForeignRule::Restrict => quote!(mensula::ForeignRule::Restrict),
    };
    tokens.extend(q);
  }
}

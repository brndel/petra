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
      ForeignRule::Cascade => quote!(data::ForeignRule::Cascade),
      ForeignRule::SetNull => quote!(data::ForeignRule::SetNull),
      ForeignRule::Restrict => quote!(data::ForeignRule::Restrict),
    };
    tokens.extend(q);
  }
}

use quote::ToTokens;

use super::ForeignReference;

#[derive(Clone)]
pub struct Modifier {
  pub unique: bool,
  pub primary: bool,
  pub reference: Option<ForeignReference>
}

impl Modifier {
  pub fn new() -> Self {
    Self {
      unique: false,
      primary: false,
      reference: None,
    }
  }
}

impl ToTokens for Modifier {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let unique = &self.unique;
    let primary = &self.primary;
    let reference = if let Some(reference) = &self.reference {
      quote!(Some(#reference))
    } else {
      quote!(None)
    };
    tokens.extend(quote!(
      data::Modifier::new( #unique, #primary, #reference)
    ));
  }
}

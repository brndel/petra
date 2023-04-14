use quote::ToTokens;
use syn::Type;

use super::ForeignRule;

#[derive(Clone)]
pub struct ForeignReference {
  pub ty: Type,
  pub on_update: ForeignRule,
  pub on_delete: ForeignRule,
  pub is_link: bool,
}

impl ForeignReference {
  pub fn new(ty: Type) -> Self {
    Self {
      ty,
      on_update: ForeignRule::default(),
      on_delete: ForeignRule::default(),
      is_link: false,
    }
  }

  pub fn link(ty: Type) -> Self {
    Self {
      ty,
      on_update: ForeignRule::default(),
      on_delete: ForeignRule::default(),
      is_link: true,
    }
  }
}

impl ToTokens for ForeignReference {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let ty = &self.ty;
    let on_update = &self.on_update;
    let on_delete = &self.on_delete;
    tokens.extend(quote!(
      data::ForeignReference::new(
        #ty::table_name(),
        #on_update,
        #on_delete
      )
    ));
  }
}

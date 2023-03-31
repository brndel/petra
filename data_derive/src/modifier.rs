use quote::ToTokens;
use syn::Type;

#[derive(Clone)]
pub struct Modifier {
  pub unique: bool,
  pub primary: bool,
  pub reference: Option<ForeignReference>,
  pub link: Option<Type>,
}

impl Modifier {
  pub fn new() -> Self {
    Self {
      unique: false,
      primary: false,
      reference: None,
      link: None,
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

#[derive(Clone)]
pub struct ForeignReference {
  pub ty: Type,
  pub on_update: ForeignRule,
  pub on_delete: ForeignRule,
}

impl ForeignReference {
  pub fn new(ty: Type) -> Self {
    Self {
      ty,
      on_update: ForeignRule::default(),
      on_delete: ForeignRule::default(),
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

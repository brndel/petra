use quote::ToTokens;
use syn::{Error, Expr, Ident, Lit};
use syn::{Field, Type};

use crate::modifier::{ForeignReference, ForeignRule, Modifier};

#[derive(Clone)]
pub struct Column {
  pub ident: Ident,
  pub field_type: Type,
  pub modifier: Modifier,
  pub field: Field,
}

impl Column {
  pub fn is_primary(&self) -> bool {
    self.modifier.primary
  }
  pub fn parse(mut field: Field, primary_field: &mut Option<Ident>) -> Result<Self, Error> {
    let ident = field.ident.clone().unwrap();
    let field_type = field.ty.clone();

    let mut modifier = Modifier::new();

    for attr in &field.attrs {
      let attr_ident = attr.path().get_ident();
      if let Some(attr_ident) = attr_ident {
        let name = attr_ident.to_string();

        match name.as_str() {
          "primary" => {
            if let Some(field) = primary_field {
              Err(Error::new_spanned(
                attr_ident,
                format!("Primary field already defined ('{}')", field.to_string()),
              ))?;
            }
            modifier.primary = true;
            *primary_field = Some(ident.to_owned());
          }
          "unique" => {
            modifier.unique = true;
          }
          "foreign_link" => {
            if let Some(_) = modifier.reference {
              Err(Error::new_spanned(
                attr_ident,
                format!("Foreign type or Link already defined"),
              ))?;
            }
            let ty: Type = attr.parse_args()?;
            modifier.link = Some(ty.clone());
            modifier.reference = Some(ForeignReference::new(ty));
          }
          "foreign" => {
            if let Some(_) = modifier.reference {
              Err(Error::new_spanned(
                attr_ident,
                format!("Foreign type already defined"),
              ))?;
            }
            let ty: Type = attr.parse_args()?;
            modifier.reference = Some(ForeignReference::new(ty));
          }
          method @ "on_update" | method @ "on_delete" => {
            static INVALID_ARG_MSG: &str =
              "expected str literal of 'cascade', 'set null' or 'restrict'";
            let reference = modifier.reference;

            if reference.is_none() {
              return Err(Error::new_spanned(
                attr_ident,
                "This field does not have a foreign attr",
              ));
            }

            let mut reference = reference.unwrap();

            let expr: Expr = attr.parse_args()?;
            match expr {
              Expr::Lit(expr) => match expr.lit {
                Lit::Str(lit) => {
                  let rule = lit.value();
                  let rule = match rule.as_str() {
                    "cascade" => ForeignRule::Cascade,
                    "set null" => ForeignRule::SetNull,
                    "restrict" => ForeignRule::Restrict,
                    _ => return Err(Error::new_spanned(lit, INVALID_ARG_MSG)),
                  };
                  match method {
                    "on_update" => {
                      reference.on_update = rule;
                    }
                    "on_delete" => {
                      reference.on_delete = rule;
                    }
                    _ => {}
                  }

                  modifier.reference = Some(reference);
                }
                _ => return Err(Error::new_spanned(expr, INVALID_ARG_MSG)),
              },
              _ => return Err(Error::new_spanned(expr, INVALID_ARG_MSG)),
            }
          }
          _ => {}
        }
      }
    }

    field.attrs.clear();

    Ok(Self {
      ident,
      field_type,
      modifier,
      field,
    })
  }
}

impl ToTokens for Column {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let name = &self.ident.to_string();
    let field_type = &self.field_type;
    let modifier = &self.modifier;

    tokens.extend(quote!(
      data::Column::new(
        #name,
        <#field_type as data::AsDataType>::as_data_type(),
        #modifier,
      )
    ))
  }
}

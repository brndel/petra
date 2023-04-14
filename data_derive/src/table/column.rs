use quote::ToTokens;
use syn::{Attribute, Error, Expr, Ident, Lit};
use syn::{Field, Type};

use super::{ForeignReference, ForeignRule, Modifier};

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

  fn handle_primary(modifier: &mut Modifier) {
    modifier.primary = true;
  }

  fn set_primary(&self, primary: &mut Option<Column>) -> Result<(), Error> {
    if self.modifier.primary {
      if let Some(primary_column) = primary {
        return Err(Error::new_spanned(
          self.ident.clone(),
          format!(
            "Primary field already defined ('{}')",
            primary_column.ident.to_string()
          ),
        ));
      } else {
        *primary = Some(self.clone());
      }
    }
    Ok(())
  }

  fn handle_unique(modifier: &mut Modifier) {
    modifier.unique = true;
  }

  fn get_foreign_type(modifier: &Modifier, attr: &Attribute) -> Result<Type, Error> {
    if let Some(_) = modifier.reference {
      Err(Error::new_spanned(
        attr,
        format!("Foreign type or Link already defined"),
      ))
    } else {
      let ty: Type = attr.parse_args()?;
      Ok(ty)
    }
  }

  fn handle_foreign(modifier: &mut Modifier, attr: &Attribute) -> Result<(), Error> {
    let ty = Self::get_foreign_type(modifier, attr)?;
    modifier.reference = Some(ForeignReference::new(ty));
    Ok(())
  }

  fn handle_foreign_link(modifier: &mut Modifier, attr: &Attribute) -> Result<(), Error> {
    let ty = Self::get_foreign_type(modifier, attr)?;
    modifier.reference = Some(ForeignReference::link(ty));
    Ok(())
  }

  fn handle_foreign_rule(
    rule_ref: &mut Option<&mut ForeignRule>,
    attr: &Attribute,
  ) -> Result<(), Error> {
    let rule_ref: &mut ForeignRule = rule_ref
      .as_mut()
      .ok_or_else(|| Error::new_spanned(attr, "This field does not have a foreign reference"))?;

    let expr: Expr = attr.parse_args()?;

    if let Expr::Lit(expr) = expr.clone() {
      if let Lit::Str(literal) = expr.lit {
        let rule = literal.value();

        let rule = match rule.as_str() {
          "cascade" => Some(ForeignRule::Cascade),
          "set null" => Some(ForeignRule::SetNull),
          "restrict" => Some(ForeignRule::Restrict),
          _ => None,
        };
        if let Some(rule) = rule {
          *rule_ref = rule;

          return Ok(());
        }
      }
    }

    Err(Error::new_spanned(
      expr,
      "Expected string literal of 'cascade', 'set null' or 'restrict'",
    ))
  }

  pub fn parse(mut field: Field, primary: &mut Option<Column>) -> Result<Self, Error> {
    let field_ident = field.ident.clone().unwrap();
    let field_type = field.ty.clone();

    let mut modifier = Modifier::new();

    for attr in &field.attrs {
      let attr_ident = attr.path().get_ident();
      if let Some(attr_ident) = attr_ident {
        let name = attr_ident.to_string();

        match name.as_str() {
          "primary" => Self::handle_primary(&mut modifier),
          "unique" => Self::handle_unique(&mut modifier),
          "foreign_link" => Self::handle_foreign_link(&mut modifier, attr)?,
          "foreign" => Self::handle_foreign(&mut modifier, attr)?,
          "on_update" => Self::handle_foreign_rule(
            &mut modifier.reference.as_mut().map(|r| &mut r.on_update),
            attr,
          )?,
          "on_delete" => Self::handle_foreign_rule(
            &mut modifier.reference.as_mut().map(|r| &mut r.on_delete),
            attr,
          )?,
          _ => {}
        }
      }
    }

    field.attrs.clear(); // Clear the attrs of the field, so the field can be used in the Insert struct

    let column = Self {
      ident: field_ident,
      field_type,
      modifier,
      field,
    };

    column.set_primary(primary)?;

    Ok(column)
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

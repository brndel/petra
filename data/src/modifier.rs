pub struct Modifier {
  unique: bool,
  primary: bool,
  reference: Option<ForeignReference>,
}

impl Modifier {
  pub fn new(unique: bool, primary: bool, reference: Option<ForeignReference>) -> Self {
    Self {
      unique,
      primary,
      reference,
    }
  }
}

pub struct ForeignReference {
  table_name: &'static str,
  on_update: ForeignRule,
  on_delete: ForeignRule,
}

impl ForeignReference {
  pub fn new(table_name: &'static str, on_update: ForeignRule, on_delete: ForeignRule) -> Self {
    Self {
      table_name,
      on_update,
      on_delete,
    }
  }
}

pub enum ForeignRule {
  Cascade,
  SetNull,
  Restrict,
}

impl AsRef<str> for ForeignRule {
  fn as_ref(&self) -> &str {
    match self {
      ForeignRule::Cascade => "CASCADE",
      ForeignRule::SetNull => "SET NULL",
      ForeignRule::Restrict => "RESTRICT",
    }
  }
}

impl ToString for Modifier {
  fn to_string(&self) -> String {
    format!(
      "{}{}{}",
      if self.unique { " UNIQUE" } else { "" },
      if self.primary { " PRIMARY KEY" } else { "" },
      if let Some(reference) = &self.reference {
        format!(
          " REFERENCES {} ON UPDATE {} ON DELETE {}",
          reference.table_name,
          reference.on_update.as_ref(),
          reference.on_delete.as_ref()
        )
      } else {
        "".to_string()
      }
    )
  }
}

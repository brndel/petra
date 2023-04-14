use std::fmt::Display;

pub struct Modifier {
  pub unique: bool,
  pub primary: bool,
  pub reference: Option<ForeignReference>,
}

impl Modifier {
  pub const fn new(unique: bool, primary: bool, reference: Option<ForeignReference>) -> Self {
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
  pub const fn new(
    table_name: &'static str,
    on_update: ForeignRule,
    on_delete: ForeignRule,
  ) -> Self {
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

impl Display for Modifier {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.unique {
      write!(f, " UNIQUE")?;
    }
    if self.primary {
      write!(f, " PRIMARY KEY")?;
    }
    if let Some(reference) = &self.reference {
      write!(
        f,
        " REFERENCES {} ON UPDATE {} ON DELETE {}",
        reference.table_name,
        reference.on_update.as_ref(),
        reference.on_delete.as_ref()
      )?;
    }
    Ok(())
  }
}
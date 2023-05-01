use std::{fmt::Display, num::ParseIntError, str::FromStr};

use chrono::NaiveDate;

pub struct ExactMonth {
  pub year: i32,
  pub month: u32,
}

impl ExactMonth {
  pub fn get_start_date(&self) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(self.year, self.month, 1)
  }

  pub fn get_end_date(&self) -> Option<NaiveDate> {
    NaiveDate::from_ymd_opt(self.year, self.month + 1, 1)
      .or(NaiveDate::from_ymd_opt(self.year + 1, 1, 1))?
      .pred_opt()
  }
}

pub enum ExactMonthParseError {
  Month(ParseIntError),
  InvalidMonth,
  Year(ParseIntError),
  TooManyParts,
  NotEnoughParts,
}

impl FromStr for ExactMonth {
  type Err = ExactMonthParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split("-");

    let year = parts.next().ok_or(ExactMonthParseError::NotEnoughParts)?;
    let month = parts.next().ok_or(ExactMonthParseError::NotEnoughParts)?;
    if parts.next().is_some() {
      return Err(ExactMonthParseError::TooManyParts);
    }

    let year = year.parse().map_err(|e| ExactMonthParseError::Year(e))?;
    let month = month.parse().map_err(|e| ExactMonthParseError::Month(e))?;

    if month < 1 || month > 12 {
      return Err(ExactMonthParseError::InvalidMonth);
    }

    Ok(Self { year, month })
  }
}

impl Display for ExactMonth {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{:0>2}", self.year, self.month)
  }
}

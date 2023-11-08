use std::{borrow::Cow, fmt::Display, str::FromStr, num::ParseIntError};

use serde::{Deserialize, Serialize};

use crate::component::{field::choice_field::Choose, select_menu::MenuItem};

use super::lang::{Lang, Translate};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Translate for Month {
    fn translate(&self, lang: &Lang) -> Cow<'static, str> {
        self.get_name(lang).into()
    }
}

impl Month {
    pub fn get_number(&self) -> u8 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }

    pub fn get_all() -> &'static [Self] {
        &[
            Month::January,
            Month::February,
            Month::March,
            Month::April,
            Month::May,
            Month::June,
            Month::July,
            Month::August,
            Month::September,
            Month::October,
            Month::November,
            Month::December,
        ]
    }

    fn get_name(&self, lang: &Lang) -> &'static str {
        match (self, lang) {
            (Month::January, Lang::German) => "Januar",
            (Month::January, Lang::English) => "January",
            (Month::February, Lang::German) => "Februar",
            (Month::February, Lang::English) => "February",
            (Month::March, Lang::German) => "März",
            (Month::March, Lang::English) => "March",
            (Month::April, Lang::German) => "April",
            (Month::April, Lang::English) => "April",
            (Month::May, Lang::German) => "Mai",
            (Month::May, Lang::English) => "May",
            (Month::June, Lang::German) => "Juni",
            (Month::June, Lang::English) => "June",
            (Month::July, Lang::German) => "Juli",
            (Month::July, Lang::English) => "July",
            (Month::August, Lang::German) => "August",
            (Month::August, Lang::English) => "August",
            (Month::September, Lang::German) => "September",
            (Month::September, Lang::English) => "September",
            (Month::October, Lang::German) => "Oktober",
            (Month::October, Lang::English) => "October",
            (Month::November, Lang::German) => "November",
            (Month::November, Lang::English) => "November",
            (Month::December, Lang::German) => "Dezember",
            (Month::December, Lang::English) => "December",
        }
    }
}

impl Choose for Month {
    fn options() -> &'static [Self] {
        Self::get_all()
    }
}

impl From<Month> for MenuItem<Month> {
    fn from(value: Month) -> Self {
        MenuItem::new(value, value.translate_default())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseMonthError {
    ParseNumber(ParseIntError),
    InvalidMonth,
}

impl TryFrom<u8> for Month {
    type Error = ParseMonthError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let month = match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => return Err(ParseMonthError::InvalidMonth),
        };

        Ok(month)
    }
}


impl FromStr for Month {
    type Err = ParseMonthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let month = s.parse::<u8>().map_err(ParseMonthError::ParseNumber)?;

        let month = Self::try_from(month)?;

        Ok(month)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonthDate {
    pub year: i32,
    pub month: Month,
}

impl MonthDate {
    pub fn new(year: i32, month: Month) -> Self {
        Self {
            year,
            month
        }
    }
}

impl Translate for MonthDate {
    fn translate(&self, lang: &Lang) -> Cow<'static, str> {
        format!("{} {:04}", self.month.translate(lang), self.year).into()
    }
}

impl MonthDate {
    pub fn as_string(&self) -> String {
        format!("{:04}-{:02}", self.year, self.month.get_number())
    }
}

impl Display for MonthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month.get_number())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParseMonthDateErr {
    NoYear,
    NoMonth,
    ParseYear(ParseIntError),
    ParseMonth(ParseMonthError),
}

impl FromStr for MonthDate {
    type Err = ParseMonthDateErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let year = parts.next().ok_or(ParseMonthDateErr::NoYear)?;
        let month = parts.next().ok_or(ParseMonthDateErr::NoMonth)?;

        let year = year.parse().map_err(ParseMonthDateErr::ParseYear)?;
        let month = month.parse().map_err(ParseMonthDateErr::ParseMonth)?;

        Ok(Self { year, month })
    }
}

use std::borrow::Cow;

use chrono::{DateTime, TimeZone, Datelike, Timelike};

#[allow(dead_code)]
pub enum Lang {
    German,
    English
}

impl Default for Lang {
    fn default() -> Self {
        Self::German
    }
}

pub trait Translate {
    fn translate(&self, lang: &Lang) -> Cow<'static, str>;
    fn translate_default(&self) -> Cow<'static, str> {
        self.translate(&Lang::default())
    }
}

impl<Tz: TimeZone> Translate for DateTime<Tz> {
    fn translate(&self, _: &Lang) -> Cow<'static, str> {
        format!(
            "{}.{} {:02}:{:02}",
            self.day(),
            self.month(),
            self.hour(),
            self.minute()
        ).into()
    }
}
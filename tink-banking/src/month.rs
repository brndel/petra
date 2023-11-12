use chrono::NaiveDate;

pub struct TinkMonth {
    pub year: i32,
    pub month: u32,
}

impl TinkMonth {
    pub fn get_first_day(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month, 1)
    }

    pub fn get_last_day(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month + 1, 1)
        .or(NaiveDate::from_ymd_opt(self.year + 1, 1, 1))?
        .pred_opt()
    }
}

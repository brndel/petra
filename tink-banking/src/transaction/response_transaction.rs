use std::num::ParseIntError;

use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTransaction {
    pub amount: Amount,
    pub descriptions: Descriptions,
    pub dates: Dates,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Amount {
    // currency_code: String,
    pub value: AmountValue,
}

impl TryInto<i64> for Amount {
    type Error = ParseIntError;

    fn try_into(self) -> Result<i64, Self::Error> {
        let unscaled = self.value.unscaled_value.parse::<i64>()?;
        let scale = 2 - self.value.scale.parse::<u32>()?;

        Ok(unscaled * 10_i64.pow(scale))
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmountValue {
    pub scale: String,
    pub unscaled_value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub booked: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Descriptions {
    // detailed: Option<TinkDetailedDescriptions>,
    pub display: String,
    pub original: String,
}
use mensula_key::Key;
use serde::{Deserialize, Serialize};

use crate::{
    api::tink::TinkPayment,
    component::{field::choice_field::Choose, select_menu::MenuItem},
    util::search::search_str,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Rule {
    pub id: Key,
    pub name: String,
    pub keywords: Vec<String>,
    pub categories: Vec<Key>,
    pub share_rule: ShareRule,
}

impl Rule {
    pub fn find_rule<'a, 'b>(rules: &'a [Rule], payment: &'b TinkPayment) -> Option<&'a Rule> {
        for rule in rules {
            if rule.matches_payment(payment) {
                return Some(rule);
            }
        }

        return None;
    }

    fn matches_payment(&self, payment: &TinkPayment) -> bool {
        for keyword in &self.keywords {
            for query in payment.get_rule_strings() {
                if search_str(&query, keyword) {
                    return true;
                }
            }
        }

        return false;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ShareRule {
    NotShared,
    Shared,
    Choose,
}

impl From<ShareRule> for i64 {
    fn from(value: ShareRule) -> Self {
        match value {
            ShareRule::NotShared => 0,
            ShareRule::Shared => 1,
            ShareRule::Choose => 2,
        }
    }
}

impl TryFrom<i64> for ShareRule {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShareRule::NotShared),
            1 => Ok(ShareRule::Shared),
            2 => Ok(ShareRule::Choose),
            _ => Err(()),
        }
    }
}

impl Choose for ShareRule {
    fn options() -> &'static [Self] {
        use ShareRule::*;
        &[Shared, NotShared, Choose]
    }
}

impl From<ShareRule> for MenuItem<ShareRule> {
    fn from(value: ShareRule) -> Self {
        let name = match value {
            ShareRule::NotShared => "Nicht geteilt",
            ShareRule::Shared => "Geteilt",
            ShareRule::Choose => "Auswählen",
        };

        let icon = match value {
            ShareRule::NotShared => "users-slash",
            ShareRule::Shared => "users",
            ShareRule::Choose => "hand-pointer",
        };

        MenuItem::with_icon(value, name, icon)
    }
}

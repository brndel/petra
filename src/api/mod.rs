pub mod category;
pub mod payment;
pub mod rule;
pub mod tink;
pub mod user;

#[cfg(feature = "ssr")]
pub mod migrate;


#[cfg(feature = "ssr")]
use mensula::sqlite;
#[cfg(feature = "ssr")]
use mensula::Database;

#[cfg(feature = "ssr")]
pub fn register_tables(db: &mut Database) -> sqlite::Result<()> {

    use self::user::server::User;
    use self::payment::server::{Payment, PaymentUserLink, PaymentCategoryLink};
    use self::category::server::{CategoryGroup, Category};
    use self::rule::server::{Rule, RuleCategoryLink, RuleKeyword};
    use self::tink::server::{TinkPayment, TinkToken};

    db.register::<User>()?;
    db.register::<CategoryGroup>()?;
    db.register::<Category>()?;
    db.register::<Payment>()?;
    db.register::<PaymentCategoryLink>()?;
    db.register::<PaymentUserLink>()?;
    db.register::<Rule>()?;
    db.register::<RuleCategoryLink>()?;
    db.register::<RuleKeyword>()?;
    db.register::<TinkPayment>()?;
    db.register::<TinkToken>()?;

    Ok(())
}
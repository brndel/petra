mod config;
mod error;
mod request;
mod tables;
mod util;
mod web;

use clap::Parser;
use config::{ServerConfig, PetraArgs};
pub use error::Error;

pub use request::Request;

use data::Database;
use tables::{
    category::Category, payment::Payment, payment::PaymentCategoryLink, payment::PaymentUserLink,
    user::User,
};
use web_server::HttpServer;

use crate::{
    tables::{
        category::CategoryGroup,
        rule::{Rule, RuleCategoryLink, RuleKeyword},
        tink_token::{TinkPayment, TinkToken},
    },
    web::tink_secret::load_tink_secrets,
};

fn main() {
    let config = PetraArgs::parse();

    println!("{:?}", config);

    start_server(&config.into());
}

fn start_server(config: &ServerConfig) {
    println!("starting server with config {:?}", config);
    let port = config.port;

    let database = open_database(config);

    load_tink_secrets(&config.tink_secret_path);
    // migrate::migrate(&database);

    let server = HttpServer::new().not_found(Box::new(move |req, _| {
        web::handle(req, &database).unwrap_or_else(|e| e.into())
    }));

    println!("starting server on http://localhost:{}", port);
    server.launch(port as i32);
}

fn open_database(config: &ServerConfig) -> Database {
    let database = Database::open(&config.db_path).unwrap();

    database.create::<User>().unwrap();
    database.create::<CategoryGroup>().unwrap();
    database.create::<Category>().unwrap();
    database.create::<Payment>().unwrap();
    database.create::<PaymentCategoryLink>().unwrap();
    database.create::<PaymentUserLink>().unwrap();
    database.create::<Rule>().unwrap();
    database.create::<RuleKeyword>().unwrap();
    database.create::<RuleCategoryLink>().unwrap();
    database.create::<TinkToken>().unwrap();
    database.create::<TinkPayment>().unwrap();

    database
}
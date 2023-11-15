use petra::api;
use petra::app;
use petra::auth;
use petra::cli;
use petra::db;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::cli::CliArgs;
    use clap::Parser;

    let args = CliArgs::parse();

    db::init(
        args.db_file
            .as_ref()
            .map(String::as_str)
            .unwrap_or("data.sqlite"),
    );
    tink_banking::load_config_from_file(
        args.tink_file
            .as_ref()
            .map(String::as_str)
            .unwrap_or("tink.toml"),
    );

    if let Some(command) = args.command {
        command.run()
    } else {
        start_server().await
    }
}

#[cfg(feature = "ssr")]
async fn start_server() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use actix_web_httpauth::middleware::HttpAuthentication;
    use app::App;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    use crate::api::tink::token_callback;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    println!("starting server at 'http://{}'", addr);

    // dbg!(
    //     GetPayments::prefix(),
    //     GetPayments::url(),
    //     TinkCallback::prefix(),
    //     TinkCallback::url()
    // );

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        let auth = HttpAuthentication::basic(auth::authenticate_user);

        App::new()
            .wrap(auth)
            // special server function
            .service(token_callback)
            // server functions
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .service(favicon_svg)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.png")]
async fn favicon_svg(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.png"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "hydrate")))]
pub fn main() {
    leptos::log!("not running as server and not running in the web. Where the fuck am i?");
}

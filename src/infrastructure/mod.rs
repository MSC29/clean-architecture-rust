use std::{env, net::TcpListener};

use crate::adapters::{
    self,
    api::shared::app_state::AppState,
    spi::{
        db::{db_connection::DbConnection, dog_facts_repository::DogFactsRepository},
        http::{cat_facts_repository::CatFactsRepository, http_connection::HttpConnection},
    },
};
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};

pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init();

    let db_connection = DbConnection { db_name: db_name.to_string() };
    let http_connection = HttpConnection {};

    //TODO pass pool as property rather than to each repo
    let data = web::Data::new(AppState {
        app_name: String::from("Animal Facts API"),
        cats_repository: CatFactsRepository {
            http_connection,
            source: env::var("CATS_SOURCE").expect("CATS_SOURCE must be set"),
        },
        dogs_repository: DogFactsRepository { db_connection },
    });

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || App::new().app_data(data.clone()).wrap(Logger::default()).configure(adapters::api::shared::routes::routes))
        .listen(listener)?
        .run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server)
}

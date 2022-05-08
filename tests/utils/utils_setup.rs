use actix_web::{web, App, HttpResponse, HttpServer};
use animal_facts_api::adapters::spi::http::models::{CatFactApiModel, CatFactsApiModel};
use dotenv::dotenv;
use std::net::TcpStream;
use std::{env, net::TcpListener};
use uuid::Uuid;

use crate::utils::utils_file::read_from_file;

use super::test_context::TestContextPostgreSQL;

pub fn spawn_app(db_name: &str) -> String {
    dotenv::from_filename(".env.test").ok();

    // Let the OS assign a port (:0)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = animal_facts_api::run(listener, db_name).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    if let Ok(_) = TcpStream::connect("localhost:3333") {
        println!("Http source faked server already running");
    } else {
        spawn_http_spi();
    }

    format!("http://127.0.0.1:{}", port)
}

pub fn spawn_http_spi() -> String {
    dotenv::from_filename(".env.test").ok();

    async fn facts_route() -> HttpResponse {
        let json = read_from_file::<CatFactsApiModel>("tests/fixtures/cat_facts.json").unwrap();
        HttpResponse::Ok().json(json)
    }

    async fn random_fact_route() -> HttpResponse {
        HttpResponse::Ok().json(CatFactApiModel {
            fact: String::from("In the 1930s, two Russian biologists discovered that color change in Siamese kittens depend on their body temperature. Siamese cats carry albino genes that work only when the body temperature is above 98° F. If these kittens are left in a very warm room, their points won’t darken and they will stay a creamy white."),
            length: 315,
        })
    }

    let s1 = HttpServer::new(move || App::new().route("facts", web::get().to(facts_route)).route("fact", web::get().to(random_fact_route)))
        .bind("0.0.0.0:3333")
        .expect("woops")
        .run();

    let _ = tokio::spawn(s1);

    format!("http://127.0.0.1:3333")
}

pub fn setup() -> TestContextPostgreSQL {
    dotenv().ok();
    TestContextPostgreSQL::new(
        &env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        //db name cannot start with a number
        format!("test_{}", Uuid::new_v4().to_simple().to_string()).as_str(),
    )
}

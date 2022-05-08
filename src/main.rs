use dotenv::dotenv;
use std::env;
use std::net::TcpListener;

use animal_facts_api::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    run(listener, &database_name)?.await
}

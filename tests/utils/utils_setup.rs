use std::{net::TcpListener, env};
use dotenv;
use uuid::Uuid;

use super::test_context_pg::TestContextPostgreSQL;

pub fn spawn_app(db_name: &str) -> String {
    dotenv::from_filename(".env.test").ok();

    // Let the OS assign a port (:0)
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let server = identity_api::run(listener, db_name).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub fn setup() -> TestContextPostgreSQL {
    TestContextPostgreSQL::new(
        &env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        //db name cannot start with a number
        format!("test_{}", Uuid::new_v4().to_simple().to_string()).as_str(),
    )
}
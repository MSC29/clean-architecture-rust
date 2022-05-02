use crate::fixtures::fixtures_run;
use diesel::{Connection, RunQueryDsl, SqliteConnection};
use fixtures_run::execute_imports;
use identity_api::adapters::spi::db::db::DbConnection;

embed_migrations!("./migrations");

pub struct TestContextPostgreSQL {
    pub base_url: String,
    pub db_name: String,
}

impl TestContextPostgreSQL {
    pub fn new(base_url: &str, db_name: &str) -> Self {
        // First, connect to postgres db to be able to create our test database.
        let conn_create_db = SqliteConnection::establish(&db_name).expect("Cannot connect to postgres database.");

        // Create a new database for the test
        let query = diesel::sql_query(format!("CREATE DATABASE {};", db_name).as_str());
        //TODO proper error handling
        query.execute(&conn_create_db).unwrap_or_else(|_| panic!("Could not create database {}", db_name));

        // First, connect to postgres db to be able to create our testdatabase.
        let conn = SqliteConnection::establish(&format!("{}/{}", base_url, db_name)).unwrap_or_else(|_| panic!("Cannot connect to {} database", db_name));

        //ensure models are created
        #[allow(unused_must_use)]
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout());

        //insert fixtures
        let db_connection = DbConnection{db_name};
        execute_imports(&db_connection);

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}

impl Drop for TestContextPostgreSQL {
    fn drop(&mut self) {
        let postgres_url = format!("{}/postgres", self.base_url);
        let conn = SqliteConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        let disconnect_users = format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';", self.db_name);

        diesel::sql_query(disconnect_users.as_str()).execute(&conn).unwrap();

        let query = diesel::sql_query(format!("DROP DATABASE {};", self.db_name).as_str());
        //TODO proper error handling
        query.execute(&conn).unwrap_or_else(|_| panic!("Couldn't drop database {}", self.db_name));
    }
}

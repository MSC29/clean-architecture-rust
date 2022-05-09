use crate::fixtures::fixtures_run;
use animal_facts_api::adapters::spi::db::db_connection::DbConnection;
use diesel::RunQueryDsl;
use fixtures_run::execute_imports;

embed_migrations!("./migrations");

pub struct TestContextPostgreSQL {
    pub base_url: String,
    pub db_name: String,
}

impl TestContextPostgreSQL {
    pub fn new(base_url: &str, db_name: &str) -> Self {
        // connect to "postgres" db to be able to create our test database.
        let db_connection_postgres_db = DbConnection { db_name: "postgres".to_string() };
        let conn_postgres_db = db_connection_postgres_db.get_pool().get().expect("couldn't get db connection from pool");

        let query = diesel::sql_query(format!("CREATE DATABASE {};", db_name).as_str());
        query.execute(&conn_postgres_db).unwrap_or_else(|_| panic!("couldn't create database {}", db_name));

        // connect to the "test" db created above
        let db_connection_test_db = DbConnection { db_name: db_name.to_string() };
        let conn_test_db = db_connection_test_db.get_pool().get().expect("couldn't get db connection from pool");

        // create data model
        embedded_migrations::run_with_output(&conn_test_db, &mut std::io::stdout()).expect("couldn't run migration");

        // insert fixtures
        execute_imports(&db_connection_test_db);

        Self {
            base_url: base_url.to_string(),
            db_name: db_name.to_string(),
        }
    }
}

impl Drop for TestContextPostgreSQL {
    fn drop(&mut self) {
        // connect to "postgres" db to be able to drop our "tests" databases.
        let db_connection_postgres_db = DbConnection { db_name: "postgres".to_string() };
        let conn_postgres_db = db_connection_postgres_db.get_pool().get().expect("couldn't get db connection from pool");

        // disconnect user before dropping the db
        let disconnect_users = format!("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';", self.db_name);
        diesel::sql_query(disconnect_users.as_str()).execute(&conn_postgres_db).unwrap();

        // drop!
        let query = diesel::sql_query(format!("DROP DATABASE {};", self.db_name).as_str());
        query.execute(&conn_postgres_db).unwrap_or_else(|_| panic!("couldn't drop test database {}", self.db_name));
    }
}

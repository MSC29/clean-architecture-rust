use diesel::{insert_into, pg::PgConnection, RunQueryDsl};
use identity_api::adapters::spi::db::{schema::{dog_facts::dsl::*}, db::DbConnection, models::DogFact};
use r2d2::PooledConnection;

use crate::{
    fixtures::fixtures_struct::{DogFactJson},
    utils::utils_file::read_user_from_file,
};

pub fn execute_imports(conn: &DbConnection) {
    
    import_targets_fixtures(conn);
}

fn import_targets_fixtures(conn: &DbConnection) {
    let json = read_user_from_file::<Vec<DogFactJson>>("tests/fixtures/dog_facts.json").unwrap();
    
    let conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(dog_facts).values(&json).execute(&conn).unwrap();
}
use animal_facts_api::adapters::spi::db::db::DbConnection;
use animal_facts_api::adapters::spi::db::schema::dog_facts::dsl::*;
use diesel::{insert_into, RunQueryDsl};

use crate::{fixtures::fixtures_struct::DogFactJson, utils::utils_file::read_from_file};

pub fn execute_imports(conn: &DbConnection) {
    import_dog_facts_fixtures(conn);
}

fn import_dog_facts_fixtures(conn: &DbConnection) {
    let json = read_from_file::<Vec<DogFactJson>>("tests/fixtures/dog_facts.json").unwrap();

    let conn = conn.get_pool().get().expect("couldn't get db connection from pool");
    insert_into(dog_facts).values(&json).execute(&conn).unwrap();
}

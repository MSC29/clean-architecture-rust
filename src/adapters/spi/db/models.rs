use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName)]
#[table_name = "dog_facts"]
pub struct DogFact {
    pub id: i32,
    pub fact: String,
}

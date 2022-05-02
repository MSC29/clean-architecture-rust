use diesel::Insertable;
use identity_api::adapters::spi::db::schema::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Debug)]
#[table_name = "dog_facts"]
pub struct DogFactJson {
    pub id: i32,
    pub fact: String,
}
use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::adapters::spi::db::{db_connection::DbConnection, db_mappers::DogFactDbMapper, models::DogFact, schema::dog_facts::dsl::*};
use crate::application::{mappers::db_mapper::DbMapper, repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract};
use crate::domain::dog_fact_entity::DogFactEntity;

pub struct DogFactsRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl DogFactsRepositoryAbstract for DogFactsRepository {
    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let result = dog_facts.filter(id.eq(dog_fact_id)).get_result::<DogFact>(&conn);

        match result {
            Ok(db_obj) => Ok(DogFactDbMapper::to_entity(db_obj)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let results = dog_facts.load::<DogFact>(&conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(|db_obj| DogFactDbMapper::to_entity(db_obj)).collect::<Vec<DogFactEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

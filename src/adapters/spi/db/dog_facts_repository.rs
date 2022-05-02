use crate::application::repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract;
use crate::{
    domain::dog_fact_entity::{DogEntities, DogFactEntity},
};
use crate::adapters::spi::db::schema::dog_facts::dsl::*;
use std::error::Error;

use actix_web::web;
use async_trait::async_trait;
use diesel::sql_types::{Integer, BigInt};
use diesel::{insert_into, prelude::*, r2d2::ConnectionManager, sql_query};
use r2d2::PooledConnection;

use super::db::DbConnection;
use super::models::DogFact;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DogFactsRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl DogFactsRepositoryAbstract for DogFactsRepository {
    async fn get_dog_fact_by_id(&self, dog_fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        
        //TODO need to find a way not to use web, since it's not very CC
        let results = web::block(move || 
            dog_facts.filter(id.eq(dog_fact_id))
            .get_result::<DogFact>(&conn)
            .expect("Error loading posts")
        ).await;
        
        //TODO Error Handling
        let results_ok = results.unwrap();
        
        Ok(DogFactEntity::new(results_ok.id, results_ok.fact))
    }

    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>>{
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        //TODO need to find a way not to use web, since it's not very CC
        let results = web::block(move || 
            dog_facts
            .load::<DogFact>(&conn)
            .expect("Error loading posts")
        ).await;
        
        //TODO Error Handling
        let results_ok = results.unwrap();
        
        Ok(results_ok.into_iter().map(|n| DogFactEntity::new(n.id, n.fact)).collect::<DogEntities>())
    }    
}

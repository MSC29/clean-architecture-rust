use crate::domain::dog_fact_entity::DogFactEntity;
use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait DogFactsRepositoryAbstract {
    async fn get_dog_fact_by_id(&self, fact_id: i32) -> Result<DogFactEntity, Box<dyn Error>>;
    async fn get_all_dog_facts(&self) -> Result<Vec<DogFactEntity>, Box<dyn Error>>;
}

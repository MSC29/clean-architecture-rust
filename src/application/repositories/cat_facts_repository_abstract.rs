use crate::domain::cat_fact_entity::{CatFactEntity};
use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait CatFactsRepositoryAbstract {
    async fn get_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>>;
    async fn get_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>>;
}

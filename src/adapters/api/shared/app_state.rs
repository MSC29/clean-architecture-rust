use crate::adapters::spi::{http::cat_facts_repository::CatFactsRepository, db::dog_facts_repository::DogFactsRepository};


pub struct AppState {
    pub app_name: String,
    pub cats_repository: CatFactsRepository,
    pub dogs_repository: DogFactsRepository,
}
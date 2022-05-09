use crate::adapters::spi::{db::dog_facts_repository::DogFactsRepository, http::cat_facts_repository::CatFactsRepository};

pub struct AppState {
    pub app_name: String,
    pub cats_repository: CatFactsRepository,
    pub dogs_repository: DogFactsRepository,
}

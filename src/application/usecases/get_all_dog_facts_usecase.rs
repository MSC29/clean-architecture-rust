use crate::{
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogEntities, error::ApiError},
};

pub struct GetAllDogFactsUseCase<'a> {
    repository: &'a dyn DogFactsRepositoryAbstract,
}

//TODO try to implement AbstractUseCase
impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        GetAllDogFactsUseCase { repository }
    }

    pub async fn execute(&self) -> Result<DogEntities, ApiError> {
        let dog_facts = self.repository.get_all_dog_facts().await;

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all dog facts", Some(e))),
        }
    }
}

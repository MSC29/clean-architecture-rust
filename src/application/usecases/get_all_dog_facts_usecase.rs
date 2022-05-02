use crate::{
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, dog_fact_entity::DogEntities},
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
        let targets = self.repository.get_all_dog_facts().await;

        match targets {
            Ok(targets) => Ok(targets),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all dog facts", Some(e))),
        }
    }
}

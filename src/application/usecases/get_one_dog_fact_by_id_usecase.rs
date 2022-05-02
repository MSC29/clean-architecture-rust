use crate::{
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};

pub struct GetOneDogFactByIdUseCase<'a> {
    dog_fact_id: &'a i32,
    repository: &'a dyn DogFactsRepositoryAbstract,
}

//TODO try to implement AbstractUseCase
impl<'a> GetOneDogFactByIdUseCase<'a> {
    pub fn new(dog_fact_id: &'a i32, repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        GetOneDogFactByIdUseCase { dog_fact_id, repository }
    }

    pub async fn execute(&self) -> Result<DogFactEntity, ApiError> {
        let dog_fact = self.repository.get_dog_fact_by_id(self.dog_fact_id.clone()).await;

        match dog_fact {
            Ok(dog_fact) => Ok(dog_fact),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get single dog fact", Some(e))),
        }
    }
}

use crate::{
    application::{repositories::{cat_facts_repository_abstract::CatFactsRepositoryAbstract}, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError, cat_fact_entity::CatFactEntity},
};

pub struct GetOneRandomCatFactUseCase<'a> {
    repository: &'a dyn CatFactsRepositoryAbstract,
}

//TODO try to implement AbstractUseCase
impl<'a> GetOneRandomCatFactUseCase<'a> {
    pub fn new(repository: &'a dyn CatFactsRepositoryAbstract) -> Self {
        GetOneRandomCatFactUseCase { repository }
    }

    pub async fn execute(&self) -> Result<CatFactEntity, ApiError> {
        let identity_result = self.repository.get_cat_fact().await;

        match identity_result {
            Ok(identity) => Ok(identity),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get random cat fact", Some(e))),
        }
    }
}

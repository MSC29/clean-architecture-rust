use crate::{
    application::{repositories::cat_facts_repository_abstract::CatFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{cat_fact_entity::CatFactEntity, error::ApiError},
};

pub struct GetAllCatFactsUseCase<'a> {
    repository: &'a dyn CatFactsRepositoryAbstract,
}

//TODO try to implement AbstractUseCase
impl<'a> GetAllCatFactsUseCase<'a> {
    pub fn new(repository: &'a dyn CatFactsRepositoryAbstract) -> Self {
        GetAllCatFactsUseCase { repository }
    }

    pub async fn execute(&self) -> Result<Vec<CatFactEntity>, ApiError> {
        let cat_facts = self.repository.get_cat_facts().await;

        match cat_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all cat facts", Some(e))),
        }
    }
}

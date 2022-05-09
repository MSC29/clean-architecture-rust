use crate::{
    application::{repositories::dog_facts_repository_abstract::DogFactsRepositoryAbstract, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};

pub struct GetAllDogFactsUseCase<'a> {
    repository: &'a dyn DogFactsRepositoryAbstract,
}

//TODO try to implement AbstractUseCase
impl<'a> GetAllDogFactsUseCase<'a> {
    pub fn new(repository: &'a dyn DogFactsRepositoryAbstract) -> Self {
        GetAllDogFactsUseCase { repository }
    }

    pub async fn execute(&self) -> Result<Vec<DogFactEntity>, ApiError> {
        let dog_facts = self.repository.get_all_dog_facts().await;

        match dog_facts {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all dog facts", Some(e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{application::repositories::dog_facts_repository_abstract::MockDogFactsRepositoryAbstract, domain::dog_fact_entity::DogFactEntity};
    use std::io::{Error, ErrorKind};

    #[actix_rt::test]
    async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
        // given the "all dog facts" usecase repo with an unexpected random error
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        dog_fact_repository
            .expect_get_all_dog_facts()
            .with()
            .times(1)
            .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await;

        // then exception
        assert!(data.is_err());
        let result = data.unwrap_err();
        assert_eq!("Cannot get all dog facts", result.message);
    }

    // TODO implement test test_should_return_custom_message_when_expected_repo_error when error handling improved

    #[actix_rt::test]
    async fn test_should_return_empty_list() {
        // given the "all dog facts" usecase repo returning an empty list
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        dog_fact_repository.expect_get_all_dog_facts().with().times(1).returning(|| Ok(Vec::<DogFactEntity>::new()));

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 0);
    }

    #[actix_rt::test]
    async fn test_should_return_list() {
        // given the "all dog facts" usecase repo returning a list of 2 entities
        let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
        dog_fact_repository.expect_get_all_dog_facts().with().times(1).returning(|| {
            Ok(vec![
                DogFactEntity {
                    fact_id: 1,
                    fact: String::from("fact1"),
                },
                DogFactEntity {
                    fact_id: 2,
                    fact: String::from("fact2"),
                },
            ])
        });

        // when calling usecase
        let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
        let data = get_all_dog_facts_usecase.execute().await.unwrap();

        // then assert the result is an empty list
        assert_eq!(data.len(), 2);
    }
}

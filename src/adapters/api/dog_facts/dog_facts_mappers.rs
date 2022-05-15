use crate::adapters::{api::dog_facts::dog_facts_payloads::DogFactPayload, api::dog_facts::dog_facts_presenters::DogFactPresenter};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::dog_fact_entity::DogFactEntity;

pub struct DogFactPresenterMapper {}

impl ApiMapper<DogFactEntity, DogFactPresenter, DogFactPayload> for DogFactPresenterMapper {
    fn to_api(entity: DogFactEntity) -> DogFactPresenter {
        DogFactPresenter {
            fact_id: entity.fact_id,
            txt: entity.fact,
        }
    }

    fn to_entity(_payload: DogFactPayload) -> DogFactEntity {
        panic!("not implemented");
    }
}

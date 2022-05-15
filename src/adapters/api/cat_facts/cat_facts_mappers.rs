use crate::adapters::api::cat_facts::cat_facts_payloads::CatFactPayload;
use crate::adapters::api::cat_facts::cat_facts_presenters::CatFactPresenter;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::cat_fact_entity::CatFactEntity;

pub struct CatFactPresenterMapper {}

impl ApiMapper<CatFactEntity, CatFactPresenter, CatFactPayload> for CatFactPresenterMapper {
    fn to_api(entity: CatFactEntity) -> CatFactPresenter {
        CatFactPresenter {
            fact: entity.fact_txt,
            nb_chars: entity.fact_length,
        }
    }

    fn to_entity(_payload: CatFactPayload) -> CatFactEntity {
        panic!("not implemented");
    }
}

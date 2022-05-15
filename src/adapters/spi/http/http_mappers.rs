use crate::adapters::spi::http::http_models::CatFactApiModel;
use crate::application::mappers::http_mapper::HttpMapper;
use crate::domain::cat_fact_entity::CatFactEntity;

pub struct CatFactHttpMapper {}

impl HttpMapper<CatFactEntity, CatFactApiModel> for CatFactHttpMapper {
    fn to_http(entity: CatFactEntity) -> CatFactApiModel {
        CatFactApiModel {
            fact: entity.fact_txt,
            length: entity.fact_length,
        }
    }

    fn to_entity(http_obj: CatFactApiModel) -> CatFactEntity {
        CatFactEntity {
            fact_txt: http_obj.fact,
            fact_length: http_obj.length,
        }
    }
}

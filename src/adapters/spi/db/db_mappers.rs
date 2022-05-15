use crate::adapters::spi::db::models::DogFact;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::dog_fact_entity::DogFactEntity;

pub struct DogFactDbMapper {}

impl DbMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_db(entity: DogFactEntity) -> DogFact {
        DogFact {
            id: entity.fact_id,
            fact: entity.fact,
        }
    }

    fn to_entity(model: DogFact) -> DogFactEntity {
        DogFactEntity {
            fact_id: model.id,
            fact: model.fact,
        }
    }
}

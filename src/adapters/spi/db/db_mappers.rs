use crate::adapters::spi::db::models::DogFact;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::dog_fact_entity::DogFactEntity;

pub struct DogFactDbMapper {}

impl DbMapper<DogFactEntity, DogFact> for DogFactDbMapper {
    fn to_db(entity: DogFactEntity) -> DogFact {
        return DogFact {
            id: entity.fact_id,
            fact: entity.fact,
        };
    }

    fn to_entity(db_obj: DogFact) -> DogFactEntity {
        return DogFactEntity {
            fact_id: db_obj.id,
            fact: db_obj.fact,
        };
    }
}

use serde::{Deserialize, Serialize};

use crate::domain::dog_fact_entity::DogFactEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct DogFactPresenter {
    pub fact_id: i32,
    pub txt: String,
}

//TODO mapper
impl DogFactPresenter {
    pub fn from(entity: DogFactEntity) -> DogFactPresenter {
        DogFactPresenter {
            fact_id: entity.fact_id,
            txt: entity.fact,
        }
    }
}

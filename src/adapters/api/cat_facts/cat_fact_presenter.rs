use serde::{Deserialize, Serialize};

use crate::domain::cat_fact_entity::CatFactEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactPresenter {
    pub fact: String,
    pub nb_chars: i32,
}

//TODO mapper
impl CatFactPresenter {
    pub fn from(entity: CatFactEntity) -> CatFactPresenter {
        CatFactPresenter {
            fact: entity.fact_txt,
            nb_chars: entity.fact_length,
        }
    }
}

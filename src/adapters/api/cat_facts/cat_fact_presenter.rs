use serde::{Deserialize, Serialize};

use crate::domain::cat_fact_entity::CatFactEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactPresenter {
    pub fact: String,
    pub nb_chars: i32,
}

//TODO mapper
impl CatFactPresenter {
    pub fn from(identity: CatFactEntity) -> CatFactPresenter {
        CatFactPresenter {
            fact: identity.fact_txt,
            nb_chars: identity.fact_length
        }
    }
}

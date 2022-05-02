use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactsApiModel {
    pub current_page: i32,
    pub data: Vec<CatFactApiModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatFactApiModel {
    pub fact: String,
    pub length: i32,
}

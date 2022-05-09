use async_trait::async_trait;
//TODO should use http_connection

use crate::{
    adapters::spi::http::{http::HttpConnection, models::CatFactApiModel},
    application::repositories::cat_facts_repository_abstract::CatFactsRepositoryAbstract,
    domain::cat_fact_entity::CatFactEntity,
};
use std::error::Error;

use super::models::CatFactsApiModel;

pub struct CatFactsRepository {
    pub http_connection: HttpConnection,
    pub source: String,
}

#[async_trait(?Send)]
impl CatFactsRepositoryAbstract for CatFactsRepository {
    async fn get_random_cat_fact(&self) -> Result<CatFactEntity, Box<dyn Error>> {
        print!("{}/facts", &self.source);
        let response = self.http_connection.client().get(&format!("{}/fact", &self.source)).send().await;

        match response {
            Ok(r) => {
                //TODO mapper
                let json = r.json::<CatFactApiModel>().await;

                match json {
                    Ok(j) => {
                        let entity = CatFactEntity {
                            fact_txt: j.fact,
                            fact_length: j.length,
                        };

                        Ok(entity)
                    }
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_cat_facts(&self) -> Result<Vec<CatFactEntity>, Box<dyn Error>> {
        let response = self.http_connection.client().get(&format!("{}/facts", &self.source)).send().await;

        match response {
            Ok(r) => {
                //TODO mapper
                let json = r.json::<CatFactsApiModel>().await;

                match json {
                    Ok(j) => Ok(j
                        .data
                        .iter()
                        .map(|model| CatFactEntity {
                            fact_txt: model.fact.clone(),
                            fact_length: model.length,
                        })
                        .collect::<Vec<CatFactEntity>>()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

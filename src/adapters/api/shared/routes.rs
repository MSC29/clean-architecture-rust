use crate::adapters::api::cat_facts::cat_facts_controller;
use crate::adapters::api::dog_facts::dog_facts_controller;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/dogs").configure(dog_facts_controller::routes))
        .service(web::scope("/api/v1/cats").configure(cat_facts_controller::routes));
}

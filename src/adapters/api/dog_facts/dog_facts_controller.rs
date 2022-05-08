use crate::{
    adapters::api::{
        dog_facts::dog_fact_presenter::DogFactPresenter,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::{get_all_dog_facts_usecase::GetAllDogFactsUseCase, get_one_dog_fact_by_id_usecase::GetOneDogFactByIdUseCase},
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_dog_facts).service(get_one_dog_fact_by_id);
}

#[get("/")]
async fn get_all_dog_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&data.dogs_repository);
    let dog_facts: Result<Vec<DogFactEntity>, ApiError> = get_all_dog_facts_usecase.execute().await;

    dog_facts
        .map_err(ErrorReponse::map_io_error)
        .and_then(|facts| Ok(HttpResponse::Ok().json(facts.into_iter().map(DogFactPresenter::from).collect::<Vec<DogFactPresenter>>())))
}

#[get("/{fact_id}")]
async fn get_one_dog_fact_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorReponse> {
    let fact_id = path.into_inner().0;
    let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&fact_id, &data.dogs_repository);
    let dog_fact = get_one_dog_fact_by_id_usecase.execute().await;

    dog_fact
        .map_err(ErrorReponse::map_io_error)
        .and_then(|fact| Ok(HttpResponse::Ok().json(DogFactPresenter::from(fact))))
}

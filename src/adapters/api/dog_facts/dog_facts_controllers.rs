use crate::{
    adapters::api::{
        dog_facts::{dog_facts_mappers::DogFactPresenterMapper, dog_facts_presenters::DogFactPresenter},
        shared::{app_state::AppState, error_presenter::ErrorResponse},
    },
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{get_all_dog_facts_usecase::GetAllDogFactsUseCase, get_one_dog_fact_by_id_usecase::GetOneDogFactByIdUseCase, interfaces::AbstractUseCase},
    },
    domain::{dog_fact_entity::DogFactEntity, error::ApiError},
};
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_dog_facts).service(get_one_dog_fact_by_id);
}

#[get("/")]
async fn get_all_dog_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&data.dogs_repository);
    let dog_facts: Result<Vec<DogFactEntity>, ApiError> = get_all_dog_facts_usecase.execute().await;

    dog_facts
        .map_err(ErrorResponse::map_io_error)
        .map(|facts| HttpResponse::Ok().json(facts.into_iter().map(DogFactPresenterMapper::to_api).collect::<Vec<DogFactPresenter>>()))
}

#[get("/{fact_id}")]
async fn get_one_dog_fact_by_id(data: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<HttpResponse, ErrorResponse> {
    let fact_id = path.into_inner().0;
    let get_one_dog_fact_by_id_usecase = GetOneDogFactByIdUseCase::new(&fact_id, &data.dogs_repository);
    let dog_fact = get_one_dog_fact_by_id_usecase.execute().await;

    dog_fact
        .map_err(ErrorResponse::map_io_error)
        .map(|fact| HttpResponse::Ok().json(DogFactPresenterMapper::to_api(fact)))
}

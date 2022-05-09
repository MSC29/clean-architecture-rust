use crate::{
    adapters::api::{
        cat_facts::cat_fact_presenter::CatFactPresenter,
        shared::{app_state::AppState, error_presenter::ErrorReponse},
    },
    application::usecases::{get_all_cat_facts_usecase::GetAllCatFactsUseCase, get_one_random_cat_fact_usecase::GetOneRandomCatFactUseCase},
};
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_cat_facts).service(get_one_random_cat_fact);
}

#[get("/")]
async fn get_all_cat_facts(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_all_cat_facts_usecase = GetAllCatFactsUseCase::new(&data.cats_repository);
    let cat_facts = get_all_cat_facts_usecase.execute().await;

    cat_facts
        .map_err(ErrorReponse::map_io_error)
        .and_then(|identities| Ok(HttpResponse::Ok().json(identities.into_iter().map(CatFactPresenter::from).collect::<Vec<CatFactPresenter>>())))
}

#[get("/random")]
async fn get_one_random_cat_fact(data: web::Data<AppState>) -> Result<HttpResponse, ErrorReponse> {
    let get_one_random_cat_fact_usecase = GetOneRandomCatFactUseCase::new(&data.cats_repository);
    let cat_fact = get_one_random_cat_fact_usecase.execute().await;

    cat_fact
        .map_err(ErrorReponse::map_io_error)
        .and_then(|fact| Ok(HttpResponse::Ok().json(CatFactPresenter::from(fact))))
}

use crate::utils::utils_setup::{setup, spawn_app};
use animal_facts_api::adapters::api::dog_facts::dog_facts_presenters::DogFactPresenter;

#[actix_rt::test]
async fn test_should_return_multiple_results() {
    // setup
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    // given the "all dog facts" route

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/dogs/", &api_address)).await.expect("Failed to execute request.");

    // then expect 3 results (inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<Vec<DogFactPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 3);
    assert_eq!(content_json[0].txt, "Forty-five percent of U.S. dogs sleep in their owner's bed");
    assert_eq!(content_json[0].fact_id, 1);
}

#[actix_rt::test]
async fn test_should_return_one_results_only() {
    // setup
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    // given the "single dog facts" route
    let dog_fact_id: i8 = 2;

    // when getting
    let response = reqwest::get(&format!("{}/api/v1/dogs/{}", &api_address, &dog_fact_id)).await.expect("Failed to execute request.");

    // then expect 1 result (id 2 inserted in db)
    assert!(response.status().is_success());

    let content_json = response.json::<DogFactPresenter>().await.unwrap();

    assert_eq!(content_json.txt, "Seventy percent of people sign their dog's name on their holiday cards");
    assert_eq!(content_json.fact_id, 2);
}

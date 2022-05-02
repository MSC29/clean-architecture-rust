use crate::utils::{
    utils_setup::{setup, spawn_app}
};
use identity_api::adapters::api::{
    dog_facts::dog_fact_presenter::DogFactPresenter,
};

#[actix_rt::test]
async fn test_should_return_multiple_results() {
    //setup
    let _ctx = setup();
    let address = spawn_app(&_ctx.db_name);

    //given

    //when
    let response = reqwest::get(&format!("{}/dogs/", &address))
        .await
        .expect("Failed to execute request.");

    //then
    assert!(response.status().is_success());

    let content_json = response.json::<Vec<DogFactPresenter>>().await.unwrap();

    //TODO same data set as python
    assert_eq!(content_json.len(), 3);
    assert_eq!(content_json[0].fact_id, 1);
    assert_eq!(content_json[0].txt, "a");
}

#[actix_rt::test]
async fn test_should_return_one_results_only() {
    //setup
    let _ctx = setup();
    let address = spawn_app(&_ctx.db_name);

    //given
    let dog_fact_id: &str = "2";

    //when
    let response = reqwest::get(&format!("{}/dogs/{}", &address, &dog_fact_id))
        .await
        .expect("Failed to execute request.");

    //then
    assert!(!response.status().is_success());

    let content_json = response.json::<DogFactPresenter>().await.unwrap();

    //TODO same data set as python
    assert_eq!(content_json.txt, "b");
    assert_eq!(content_json.fact_id, 2);
}

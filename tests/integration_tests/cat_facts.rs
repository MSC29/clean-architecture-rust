use crate::utils::utils_setup::{setup, spawn_app};
use animal_facts_api::adapters::api::cat_facts::cat_fact_presenter::CatFactPresenter;

#[actix_rt::test]
async fn test_should_return_multiple_results() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = reqwest::get(&format!("{}/api/v1/cats/", &api_address)).await.expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content_json = response.json::<Vec<CatFactPresenter>>().await.unwrap();

    assert_eq!(content_json.len(), 10);
    assert_eq!(
        content_json[0].fact,
        "The first true cats came into existence about 12 million years ago and were the Proailurus."
    );
    assert_eq!(content_json[0].nb_chars, 91);
}

#[actix_rt::test]
async fn test_should_return_one_results_only() {
    let _ctx = setup();
    let api_address = spawn_app(&_ctx.db_name);

    let response = reqwest::get(&format!("{}/api/v1/cats/random", &api_address)).await.expect("Failed to execute request.");

    assert!(response.status().is_success());

    let content_json = response.json::<CatFactPresenter>().await.unwrap();

    assert_eq!(content_json.fact, "In the 1930s, two Russian biologists discovered that color change in Siamese kittens depend on their body temperature. Siamese cats carry albino genes that work only when the body temperature is above 98° F. If these kittens are left in a very warm room, their points won’t darken and they will stay a creamy white.");
    assert_eq!(content_json.nb_chars, 315);
}

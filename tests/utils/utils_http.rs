use httpmock::{prelude::*, Mock};

pub fn mock_http_spi(path: &str) -> Mock {
    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let hello_mock = server.mock(|when, then| {
        when.method(GET).path("/translate").query_param("word", "hello");
        then.status(200).header("content-type", "text/html; charset=UTF-8").body("Привет");
    });

    // Send an HTTP request to the mock server. This simulates your code.
    let response = isahc::get(server.url("/translate?word=hello")).unwrap();

    http_spi_mock
}

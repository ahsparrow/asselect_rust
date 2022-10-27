use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::Value as JsonValue;

pub async fn fetch_yaixm() -> Result<JsonValue, Error> {
    let result = Request::get("/yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await,
        Err(e) => Err(e),
    }
}

pub fn loa_names(yaixm: &JsonValue) -> Vec<String> {
    let loa = yaixm["loa"].as_array().unwrap();
    loa.iter()
        .filter(|x| !x["default"].as_bool().unwrap_or(false))
        .map(|x| x["name"].as_str().unwrap().to_string())
        .collect::<Vec<String>>()
}

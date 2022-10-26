use serde_json::Value as JsonValue;

pub fn loa_names(yaixm: &JsonValue) -> Vec<String> {
    let loa = yaixm["loa"].as_array().unwrap();
    loa.iter()
        .filter(|x| !x["default"].as_bool().unwrap_or(false))
        .map(|x| x["name"].as_str().unwrap().to_string())
        .collect::<Vec<String>>()
}

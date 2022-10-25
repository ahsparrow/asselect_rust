use crate::components::test;
use serde_json::Value as JsonValue;
use yew::{html, Html};

pub fn asselect(yaixm: &JsonValue) -> Html {
    let loa = yaixm["loa"].as_array().unwrap();
    let loa_names = loa
        .iter()
        .map(|x| x["name"].as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    html! {
        <div class="section">
          <div class="container">
            <test::Test yaixm={loa_names} />
          </div>
       </div>
    }
}

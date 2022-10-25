use gloo_net::http::Request;
use serde_json::Value as JsonValue;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle};

mod asselect;
mod components;

#[function_component(App)]
fn app() -> Html {
    let yaixm: UseStateHandle<Option<JsonValue>> = use_state(|| None);

    {
        let yaixm = yaixm.clone();

        // use_effect_with_deps automagically takes account of component state
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let result = Request::get("/yaixm.json").send().await;
                    match result {
                        Ok(response) => {
                            let json: JsonValue = response.json().await.unwrap();
                            yaixm.set(Some(json));
                        }
                        Err(_e) => {}
                    }
                });
                || ()
            },
            (),
        );
    }

    let html_logic = match yaixm.as_ref() {
        Some(yaixm) => asselect::asselect(yaixm),

        None => {
            html! {
                <div class="section">
                  <div class="container">
                    <div class="notification is-info">
                      <h2 class="title is-4">{"Waiting for airspace data..."}</h2>
                    </div>
                  </div>
                </div>
            }
        }
    };

    html! {
        {html_logic}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

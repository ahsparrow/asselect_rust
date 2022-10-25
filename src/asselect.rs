use crate::components::test;
use crate::yaixm;
use serde_json::Value as JsonValue;
use yew::{html, Callback, Html};

pub struct LoaSetting {
    pub name: String,
    pub value: bool
}

pub fn asselect(yaixm: &JsonValue) -> Html {

    let loa_names = yaixm::loa_names(yaixm);

    let loa_callback = Callback::from(move |loa: LoaSetting| log::info!("{} {}", loa.name, loa.value));

    html! {
        <div class="section">
          <div class="container">
            <test::Test loa={loa_names} callback={loa_callback}/>
          </div>
       </div>
    }
}

use yew::{function_component, html};

#[function_component(WavePanel)]
pub fn wave_panel() -> Html {
    html! {
        <div>
          {"Hello wave panel"}
        </div>
    }
}

use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AirspaceTab)]
pub fn airspace_tab(_props: &Props) -> Html {
    html! {
        <div>
          {"Hello airspace tab"}
        </div>
    }
}

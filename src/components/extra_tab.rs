use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(ExtraTab)]
pub fn extra_tab(props: &Props) -> Html {
    html! {
        <div>
          {"Hello extra tab"}
          { for props.children.iter() }
        </div>
    }
}

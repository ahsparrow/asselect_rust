use web_sys::HtmlElement;
use yew::{function_component, html, use_state, Callback, Children, Html, MouseEvent, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub names: Vec<String>,
}

#[function_component(ExtraTab)]
pub fn extra_tab(props: &Props) -> Html {
    let panel = use_state(|| 0);

    let onclick = {
        let panel = panel.clone();
        Callback::from(move |e: MouseEvent| {
            let id: u8 = e.target_unchecked_into::<HtmlElement>().id().parse().unwrap();
            panel.set(id)
        })
    };

    let iter = props.names.iter().zip(props.children.iter()).zip(0..);

    let panels = iter.map(|((name, child), n)| html!{
        <div class="card mb-4">
          <header class="card-header">
            <p class="card-header-title">
              { name }
            </p>
            <button id={n.to_string()} class="card-header-icon" onclick={onclick.clone()}>
              <span id={n.to_string()}>
                <i id={n.to_string()}>{ "+" }</i>
              </span>
            </button>
          </header>
          <div class="card-content" hidden={n != *panel}>
            { child }
          </div>
        </div>
    }).collect::<Html>();

    html! {
        <div>
          { panels }
        </div>
    }
}

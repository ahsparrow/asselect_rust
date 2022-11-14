use web_sys::HtmlElement;
use yew::{
    classes, function_component, html, use_state, Callback, Children, Html, MouseEvent, Properties,
    TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub tab_names: Vec<String>,
    pub children: Children,
}

#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let tab = use_state(|| "tab-0".to_string());

    let onclick = {
        let tab = tab.clone();
        Callback::from(move |e: MouseEvent| {
            let id: String = e
                .target_unchecked_into::<HtmlElement>()
                .id()
                .parse()
                .unwrap();
            tab.set(id);
        })
    };

    let tab_id = |id: usize| format!("tab-{}", id);

    let tabs = || -> Html {
        props
            .tab_names
            .iter()
            .zip(0..)
            .map(|(t, id)| {
                html! {
                    <li class={classes!((*tab == tab_id(id)).then_some("is-active"))}>
                      <a id={tab_id(id)}>
                        {t}
                      </a>
                    </li>
                }
            })
            .collect()
    };

    let panel = || -> Html {
        props
            .children
            .iter()
            .zip(0..)
            .map(|(p, id)| {
                html! {
                    <div hidden={tab_id(id) != *tab}>
                      {p}
                    </div>
                }
            })
            .collect::<Html>()
    };

    html! {
        <>
        <div class="tabs">
          <ul {onclick}>
            { tabs() }
          </ul>
        </div>
        <div class="mx-4">
          { panel() }
        </div>
        </>
    }
}

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
    let tab = use_state(|| 0);

    let onclick = {
        let tab = tab.clone();
        Callback::from(move |e: MouseEvent| {
            let id: u8 = e.target_unchecked_into::<HtmlElement>().id().parse().unwrap();
            tab.set(id);
        })
    };

    let tabs = || -> Html {
        props
            .tab_names
            .iter()
            .zip(0..)
            .map(|(t, id)| html! {<li class={classes!((*tab == id).then_some("is-active"))}><a id={id.to_string()}>{t}</a></li>})
            .collect()
    };

    let panel = || -> Html {
        props
            .children
            .iter()
            .zip(0..)
            .filter(|(_t, id)| *tab == *id)
            .map(|x| x.0)
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

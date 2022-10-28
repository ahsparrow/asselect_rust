use crate::LoaSetting;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub loa: Vec<String>,
    pub selected: HashSet<String>,
    pub callback: Callback<LoaSetting>,
}

#[function_component(Test)]
pub fn test(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let id = e.target_unchecked_into::<HtmlInputElement>().id();
        let value = e.target_unchecked_into::<HtmlInputElement>().checked();

        LoaSetting {
            id: id,
            checked: value,
        }
    });

    html! {
        <div class="columns is-multiline">
        {
            props.loa.iter().map(|name| {
                let checked = props.selected.contains(name);
                html!(
                      <div class="column is-one-third">
                        <div class="field">
                        <label class="checkbox">
                          <input type="checkbox" class="mr-1" {checked} id={name.clone()} onchange={onchange.clone()} />
                            {name}
                        </label>
                        </div>
                      </div>
                    )
            }).collect::<Html>()
        }
        </div>
    }
}

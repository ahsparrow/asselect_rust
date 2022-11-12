use crate::ExtraSetting;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category: String,
    pub names: Vec<String>,
    pub selected: HashSet<String>,
    pub callback: Callback<ExtraSetting>,
}

#[function_component(ExtraPanel)]
pub fn extra_panel(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let id = e.target_unchecked_into::<HtmlInputElement>().id();
        let checked = e.target_unchecked_into::<HtmlInputElement>().checked();

        ExtraSetting {
            id,
            checked,
        }
    });

    html! {
        <div class="columns is-multiline">
        {
            props.names.iter().map(|name| {
                let checked = props.selected.contains(name);
                let id = format!("{}-{}", props.category, name);
                html!(
                      <div class="column is-one-third">
                        <div class="field">
                        <label class="checkbox">
                          <input type="checkbox" class="mr-1" {checked} id={id} onchange={onchange.clone()} />
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

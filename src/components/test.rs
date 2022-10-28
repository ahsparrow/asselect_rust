use crate::LoaSetting;
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub loa: Vec<String>,
    pub selected: HashSet<String>,
    pub callback: Callback<LoaSetting>,
}

pub struct Test;

impl Component for Test {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.props().callback.reform(move |e: Event| {
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
                ctx.props().loa.iter().map(|name| {
                    let checked = ctx.props().selected.contains(name);
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
}

use crate::asselect::LoaSetting;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub loa: Vec<String>,
    pub callback: Callback<LoaSetting>,
}

#[function_component(Test)]
pub fn test(props: &Props) -> Html {
    let Props { loa, callback } = props.clone();

    let callback = move |e: Event| {
        let name = e.target_unchecked_into::<HtmlInputElement>().name();
        let value = e.target_unchecked_into::<HtmlInputElement>().checked();
        callback.emit(LoaSetting {
            name: name,
            value: value,
        });
    };

    html! {
      <>
        {
            loa.iter().map(|name| {
                html!(
                    <div class="field">
                    <label class="checkbox">
                      <input type="checkbox" class="mr-1" name={name.clone()} onchange={callback.clone()} />
                        {name}
                    </label>
                    </div>
                    )
            }).collect::<Html>()
        }
      </>
    }
}

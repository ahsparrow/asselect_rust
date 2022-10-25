use web_sys::HtmlInputElement;
use yew::{function_component, html, Event, Html, Properties, TargetCast};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub yaixm: Vec<String>,
}

#[function_component(Test)]
pub fn test(props: &Props) -> Html {
    let callback = |e: Event| {
        let n = e.target_unchecked_into::<HtmlInputElement>().name();
        let c = e.target_unchecked_into::<HtmlInputElement>().checked();
        log::info!("{} {}", n, c);
    };

    html! {
      <>
        {
            props.yaixm.iter().map(|name| {
                html!(
                    <div class="field">
                    <label class="checkbox">
                      <input type="checkbox" class="mr-1" name={name.clone()} onchange={callback} />
                        {name}
                    </label>
                    </div>
                    )
            }).collect::<Html>()
        }
      </>
    }
}

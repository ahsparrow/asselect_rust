use crate::ExtraCategory;
use yew::{function_component, html, use_state, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub names: Vec<String>,
    pub categories: Vec<ExtraCategory>,
    pub on_clear: Callback<ExtraCategory>,
}

#[function_component(ExtraTab)]
pub fn extra_tab(props: &Props) -> Html {
    let active_panel = use_state(|| 0);

    let onclick = {
        let active_panel = active_panel.clone();
        move |value| active_panel.set(value)
    };

    let onclear = {
        let category = props.categories[*active_panel];
        let onclear = props.on_clear.clone();
        move |_| onclear.emit(category)
    };

    let iter = props.names.iter().zip(props.children.iter()).enumerate();
    let panels = iter
        .map(|(n, (name, child))| {
            let onclick = onclick.clone();
            html! {
                <div class="card block">
                  <header class="card-header is-clickable" onclick={move |_| onclick(n)}>
                    <level class="card-header-title">
                      <p>{ name }</p>
                      {
                        if n == *active_panel {
                          html! {
                            <button class="button is-link is-light is-small ml-4" onclick={onclear.clone()}>
                              {"Clear"}
                            </button>
                            }
                        } else {
                            html! ()
                        }
                      }
                    </level>

                    {
                      if n != *active_panel {
                          html! {
                            <i class="card-header-icon">{ "+" }</i>
                          }
                      } else {
                          html! {}
                      }
                    }
                  </header>

                  <div class="card-content" hidden={n != *active_panel}>
                    { child }
                  </div>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div>
          { panels }
        </div>
    }
}

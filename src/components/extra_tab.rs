use crate::ExtraCategory;
use yew::{html, Callback, Children, Component, Context, Html, Properties};

pub enum Msg {
    Click(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub names: Vec<String>,
    pub categories: Vec<ExtraCategory>,
    pub on_clear: Callback<ExtraCategory>,
}

pub struct ExtraTab {
    active_panel: usize,
}

impl Component for ExtraTab {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ExtraTab { active_panel: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click(panel) => {
                self.active_panel = panel;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();

        let category = props.categories[self.active_panel];
        let on_clear = props.on_clear.reform(move |_| category);

        let iter = props.names.iter().zip(props.children.iter()).zip(0..);
        let panels = iter
            .map(|((name, child), n)| {
                html! {
                    <div class="card mb-4">
                      <header class="card-header is-clickable"
                              onclick={link.callback(move |_| Msg::Click(n))}>
                        <level class="card-header-title">
                          <p>{ name }</p>
                          {
                            if n == self.active_panel {
                              html! {
                                <button class="button is-link is-light is-small ml-4"
                                        onclick={on_clear.clone()}>
                                  {"Clear"}
                                </button>
                                }
                            } else {
                                html! ()
                            }
                          }
                        </level>

                        {
                          if n != self.active_panel {
                              html! {
                                <i class="card-header-icon">{ "+" }</i>
                              }
                          } else {
                              html! {}
                          }
                        }
                      </header>

                      <div class="card-content" hidden={n != self.active_panel}>
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
}

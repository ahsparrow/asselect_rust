use yew::{html, Children, Component, Context, Html, Properties};

pub enum Msg {
    Click(u8),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub names: Vec<String>,
}

pub struct ExtraTab {
    active_panel: u8,
}

impl Component for ExtraTab {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ExtraTab {
            active_panel: 0,
        }
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
        let iter = ctx.props().names.iter().zip(ctx.props().children.iter()).zip(0..);

        let panels = iter.map(|((name, child), n)| html!{
            <div class="card mb-4">
              <header class="card-header">
                <p class="card-header-title">
                  { name }
                </p>

                {
                  if n != self.active_panel {
                      html! {
                        <button class="card-header-icon" onclick={ctx.link().callback(move |_| Msg::Click(n))}>
                          <span>
                            <i>{ "+" }</i>
                          </span>
                        </button>
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
        }).collect::<Html>();

        html! {
            <div>
              { panels }
            </div>
        }
    }
}

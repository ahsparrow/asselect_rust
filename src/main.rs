use gloo_storage::{LocalStorage, Storage};
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use yew::{html, Callback, Component, Context, ContextProvider, Html};

//mod asselect;
mod components;
mod yaixm;

// For settings callback
#[derive(Debug)]
pub enum SettingCategory {
    Airspace,
    Loa,
    Rat,
    Wave
}

pub struct Setting {
    pub category: SettingCategory,
    pub id: String,
    pub value: Option<String>,
    pub checked: Option<bool>
}

// Application context
#[derive(Clone, PartialEq)]
pub struct AppContext {
    pub callback: Callback<Setting>
}

enum Msg {
    Save,
    Set(Setting),
    YaixmError,
    YaixmData(JsonValue)
}

struct App {
    yaixm: Option<JsonValue>,
    loa: HashSet<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match yaixm::fetch_yaixm().await {
                Ok(json) => Msg::YaixmData(json),
                Err(_err) => Msg::YaixmError,
            }
        });

        let loa: HashSet<String> = LocalStorage::get("loa").unwrap_or(HashSet::new());

        Self {
            yaixm: None,
            loa: loa,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::YaixmData(json) => {
                self.yaixm = Some(json);
                true
            }
            Msg::YaixmError => {
                log::error!("Can't fetch YAIXM data");
                false
            }
            Msg::Set(setting) => {
                log::info!("{:?} {} '{}' {}",
                           setting.category,
                           setting.id,
                           setting.value.unwrap_or("".to_string()),
                           setting.checked.unwrap_or(false)
                           );
                /*
                if setting.value {
                    self.loa.replace(setting.name);
                } else {
                    self.loa.remove(&setting.name);
                }
                */
                false
            }
            Msg::Save =>
            {
                log::info!("Save");
                LocalStorage::set("loa", self.loa.clone()).unwrap();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = AppContext {
            callback: ctx.link().callback(Msg::Set)
        };

        match self.yaixm.as_ref() {
            Some(yaixm) => {
                let save_callback = ctx.link().callback(|_| Msg::Save);
                html! {
                    <ContextProvider<AppContext> context={context.clone()}>
                    <div class="container">
                      <components::test::Test
                            loa={yaixm::loa_names(yaixm)}
                            selected={self.loa.clone()}
                      />
                      <button class="button is-primary" onclick={save_callback}>{"Save"}</button>
                    </div>
                    </ContextProvider<AppContext>>
                }
            }

            None => {
                html! {
                    <div class="section">
                      <div class="container">
                        <div class="notification is-info">
                          <h2 class="title is-4">{"Waiting for airspace data..."}</h2>
                        </div>
                      </div>
                    </div>
                }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

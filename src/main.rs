use components::{Tabs, AirspaceTab, ExtraTab, NotamTab, HelpTab, AirspacePanel,
                 OptionsPanel, RatPanel, LoaPanel, WavePanel};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashSet;
use yew::{html, Component, Context, Html};

mod components;
mod yaixm;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Airspace {
    atz: String,
    gliding: String,
    hirta_gvs: String,
    ils: String,
    microlight: String,
    unlicensed: String,
    home: bool,
    obstacle: bool,
}

impl Airspace {
    fn new() -> Self {
        Airspace {
            atz: "ctr".to_string(),
            gliding: "exclude".to_string(),
            hirta_gvs: "exclude".to_string(),
            ils: "atz".to_string(),
            microlight: "exclude".to_string(),
            unlicensed: "exclude".to_string(),
            home: false,
            obstacle: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Options {
    max_flight_level: u16,
    radio: bool,
    north: f64,
    south: f64,
    format: String,
}

impl Options {
    fn new() -> Self {
        Options {
            max_flight_level: 660,
            radio: false,
            north: 59.0,
            south: 49.0,
            format: "openair".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    airspace: Airspace,
    options: Options,
    loa: HashSet<String>,
    rat: HashSet<String>,
    wave: HashSet<String>,
}

impl Settings {
    fn new() -> Self {
        Settings {
            airspace: Airspace::new(),
            options: Options::new(),
            loa: HashSet::new(),
            rat: HashSet:: new(),
            wave: HashSet:: new(),
        }
    }
}

pub struct LoaSetting {
    pub id: String,
    pub checked: bool
}

// App messages
enum Msg {
    Save,
    LoaSet(LoaSetting),
    YaixmError,
    YaixmData(JsonValue),
}

// App component
struct App {
    yaixm: Option<JsonValue>,
    settings: Settings,
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

        Self {
            yaixm: None,
            settings: LocalStorage::get("settings").unwrap_or(Settings::new())
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
            Msg::LoaSet(setting) => {
                log::info!("{} {}", setting.id, setting.checked);
                if setting.checked {
                    self.settings.loa.replace(setting.id);
                } else {
                    self.settings.loa.remove(&setting.id);
                }
                false
            }
            Msg::Save =>
            {
                log::info!("Save");
                LocalStorage::set("settings", self.settings.clone()).unwrap();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.yaixm.as_ref() {
            Some(yaixm) => {
                let _loa_callback = ctx.link().callback(|s| Msg::LoaSet(s));
                let save_callback = ctx.link().callback(|_| Msg::Save);

                let _selected = self.settings.loa.clone();
                let _loa_names = yaixm::loa_names(yaixm);

                let tab_names = vec![
                    "Airspace".to_string(),
                    "Extra".to_string(),
                    "NOTAM".to_string(),
                    "Help".to_string()
                ];

                html! {
                    /*
                    <div class="container">
                      <components::test::Test callback={loa_callback} loa={loa_names} selected={selected}/>
                      <button class="button is-primary" onclick={save_callback}>{"Save"}</button>
                    </div>
                    */

                    <div class="container">
                      <Tabs {tab_names}>
                        <AirspaceTab>
                          <AirspacePanel />
                          <OptionsPanel />
                        </AirspaceTab>
                        <ExtraTab>
                          <RatPanel />
                          <LoaPanel />
                          <WavePanel />
                        </ExtraTab>
                        <NotamTab />
                        <HelpTab />
                      </Tabs>
                      <button class="button is-primary" onclick={save_callback}>{"Save"}</button>
                    </div>
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

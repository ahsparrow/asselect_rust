#![allow(clippy::let_unit_value)] // For problem in html! macro

use components::{Tabs, AirspaceTab, OptionsTab, ExtraTab, NotamTab,
                 RatPanel, LoaPanel, WavePanel};
use gloo_file::{Blob, ObjectUrl};
use gloo_storage::{LocalStorage, Storage};
use gloo_utils::document;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use yew::{html, Component, Context, Html};
use yaixm::util::{fetch_yaixm, gliding_sites, loa_names};
use yaixm::openair::openair;
use yaixm::Yaixm;

mod components;
mod yaixm;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Airspace {
    atz: String,
    ils: String,
    unlicensed: String,
    microlight: String,
    gliding: String,
    home: String,
    hirta_gvs: String,
    obstacle: String,
}

impl Default for Airspace {
    fn default() -> Self {
        Airspace {
            atz: "ctr".to_string(),
            ils: "atz".to_string(),
            unlicensed: "exclude".to_string(),
            microlight: "exclude".to_string(),
            gliding: "exclude".to_string(),
            home: "None".to_string(),
            hirta_gvs: "exclude".to_string(),
            obstacle: "exclude".to_string()
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Options {
    max_level: u16,
    radio: bool,
    north: f64,
    south: f64,
    format: String,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            max_level: 600,
            radio: false,
            north: 59.0,
            south: 49.0,
            format: "openair".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    airspace: Airspace,
    options: Options,
    loa: HashSet<String>,
    rat: HashSet<String>,
    wave: HashSet<String>,
}

pub struct AirspaceSetting {
    pub id: String,
    pub value: String,
}

pub struct LoaSetting {
    pub id: String,
    pub checked: bool
}

// App messages
enum Msg {
    Save,
    AirspaceSet(AirspaceSetting),
    LoaSet(LoaSetting),
    YaixmError,
    YaixmData(Yaixm),
}

// App component
struct App {
    yaixm: Option<Yaixm>,
    settings: Settings,
}

impl App {
    fn no_yaixm_view(&self) -> Html {
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

    fn yaixm_view(&self, ctx: &Context<Self>, yaixm: &Yaixm) -> Html {
        let airspace_callback = ctx.link().callback(Msg::AirspaceSet);
        let loa_callback = ctx.link().callback(Msg::LoaSet);
        let save_callback = ctx.link().callback(|_| Msg::Save);

        let airspace_settings = self.settings.airspace.clone();
        let airspace_options = self.settings.options.clone();

        let mut gliding_sites = gliding_sites(yaixm);
        gliding_sites.sort();

        let loa_selected = self.settings.loa.clone();
        let loa_names = loa_names(yaixm);

        let tab_names = vec![
            "Main".to_string(),
            "Options".to_string(),
            "Extra".to_string(),
            "NOTAM".to_string(),
        ];

        html! {
            <>
            <div class="hero is-small is-primary">
              <div class="hero-body py-2">
                <p class="subtitle is-4">
                  {"ASSelect - UK Airspace"}
                </p>
              </div>
            </div>
            <div class="container">
              <Tabs {tab_names}>
                <AirspaceTab settings={airspace_settings} {gliding_sites} callback={airspace_callback.clone()} />
                <OptionsTab options={airspace_options} callback={airspace_callback.clone()} />
                <ExtraTab>
                  <RatPanel />
                  <LoaPanel names={loa_names} selected={loa_selected} callback={loa_callback}/>
                  <WavePanel />
                </ExtraTab>
                <NotamTab />
              </Tabs>
            </div>
            <div class="container">
              <div class="ml-4 mt-4">
                <button class="button is-primary" onclick={save_callback}>
                  {"Save"}
                </button>
                <a id="download" hidden=true download="openair.txt">{"Download"}</a>
              </div>
            </div>
            </>
        }
    }

    fn save(&self) {
        LocalStorage::set("settings", self.settings.clone()).unwrap();

        let oa = openair(&self.yaixm.as_ref().unwrap()).join("\n");
        let blob = Blob::new(oa.as_str());
        let object_url = ObjectUrl::from(blob);

        let download_anchor = document()
            .get_element_by_id("download")
            .expect("No document")
            .dyn_into::<web_sys::HtmlAnchorElement>()
            .expect("No anchor element");

        download_anchor.set_href(&object_url);
        download_anchor.click();
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match fetch_yaixm().await {
                Ok(yaixm) => Msg::YaixmData(yaixm),
                Err(_err) => Msg::YaixmError,
            }
        });

        Self {
            yaixm: None,
            settings: LocalStorage::get("settings").unwrap_or_default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::YaixmData(yaixm) => {
                self.yaixm = Some(yaixm);
                true
            }
            Msg::YaixmError => {
                log::error!("Can't fetch YAIXM data");
                false
            }
            Msg::AirspaceSet(setting) => {
                log::info!("Setting {}: {}", setting.id, setting.value);
                let value = setting.value;
                match setting.id.as_str() {
                    "atz" => self.settings.airspace.atz = value,
                    "ils" => self.settings.airspace.ils = value,
                    "unlicensed" => self.settings.airspace.unlicensed = value,
                    "microlight" => self.settings.airspace.microlight = value,
                    "gliding" => self.settings.airspace.gliding = value,
                    "home" => self.settings.airspace.home = value,
                    "hirta_gvs" => self.settings.airspace.hirta_gvs = value,
                    "obstacle" => self.settings.airspace.obstacle = value,

                    "max_level" => self.settings.options.max_level = value.parse::<u16>().unwrap(),
                    "radio" => self.settings.options.radio = value == "yes",
                    "north" => self.settings.options.north = value.parse::<f64>().unwrap(),
                    "south" => self.settings.options.south = value.parse::<f64>().unwrap(),
                    "format" => self.settings.options.format = value,

                    _ => (),
                }
                true
            }
            Msg::LoaSet(setting) => {
                if setting.checked {
                    self.settings.loa.replace(setting.id);
                } else {
                    self.settings.loa.remove(&setting.id);
                }
                true
            }
            Msg::Save =>
            {
                self.save();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.yaixm.as_ref() {
            Some(yaixm) => self.yaixm_view(ctx, yaixm),
            None => self.no_yaixm_view(),
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

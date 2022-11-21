#![allow(clippy::let_unit_value)] // For problem in html! macro

use components::{AirspaceTab, ExtraPanel, ExtraTab, NotamTab, OptionsTab, Tabs};
use gloo_file::{Blob, ObjectUrl};
use gloo_storage::{LocalStorage, Storage};
use gloo_utils::document;
use wasm_bindgen::JsCast;
use yaixm::convert::{openair, AirType, Format, Settings};
use yaixm::util::{fetch_yaixm, gliding_sites, loa_names, rat_names, wav_names};
use yaixm::Yaixm;
use yew::{html, Component, Context, Html};

mod components;
mod yaixm;

// Application messages
pub struct AirspaceSetting {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ExtraCategory {
    Rat,
    Loa,
    Wave,
}

pub struct ExtraSetting {
    pub category: ExtraCategory,
    pub name: String,
    pub checked: bool,
}

enum Msg {
    Save,
    AirspaceSet(AirspaceSetting),
    ExtraClear(ExtraCategory),
    ExtraSet(ExtraSetting),
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
        let extra_callback = ctx.link().callback(Msg::ExtraSet);
        let save_callback = ctx.link().callback(|_| Msg::Save);

        let airspace_settings = self.settings.airspace.clone();
        let airspace_options = self.settings.options.clone();

        let mut gliding_sites = gliding_sites(yaixm);
        gliding_sites.sort();

        let rat_selected = self.settings.rat.clone();
        let rat_names = rat_names(yaixm);

        let loa_selected = self.settings.loa.clone();
        let loa_names = loa_names(yaixm);

        let wav_selected = self.settings.wave.clone();
        let mut wav_names = wav_names(yaixm);
        wav_names.sort();

        let extra_names = vec![
            "Temporary Restrictions, RA(T)".to_string(),
            "Local Agreements".to_string(),
            "Wave Boxes".to_string(),
        ];

        let tab_names = vec![
            "Main".to_string(),
            "Options".to_string(),
            "Extra".to_string(),
            "NOTAM".to_string(),
        ];

        let on_clear = ctx.link().callback(Msg::ExtraClear);

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
                <ExtraTab names={extra_names} categories={vec![ExtraCategory::Rat, ExtraCategory::Loa, ExtraCategory::Wave]} on_clear={on_clear.clone()}>
                  <ExtraPanel category={ExtraCategory::Rat} names={rat_names} selected={rat_selected} callback={extra_callback.clone()}/>
                  <ExtraPanel category={ExtraCategory::Loa} names={loa_names} selected={loa_selected} callback={extra_callback.clone()}/>
                  <ExtraPanel category={ExtraCategory::Wave} names={wav_names} selected={wav_selected} callback={extra_callback.clone()}/>
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

        let oa = openair(self.yaixm.as_ref().unwrap(), &self.settings);
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
                let value = setting.value;
                match setting.name.as_str() {
                    "atz" => self.settings.airspace.atz = value,
                    "ils" => self.settings.airspace.ils = value,
                    "unlicensed" => {
                        self.settings.airspace.unlicensed = match value.as_str() {
                            "classf" => Some(AirType::F),
                            "classg" => Some(AirType::G),
                            _ => None
                        }
                    }
                    "microlight" => {
                        self.settings.airspace.microlight = match value.as_str() {
                            "classf" => Some(AirType::F),
                            "classg" => Some(AirType::G),
                            _ => None
                        }
                    }
                    "gliding" => self.settings.airspace.gliding = value,
                    "home" => self.settings.airspace.home = value,
                    "hirta_gvs" => self.settings.airspace.hirta_gvs = value,
                    "obstacle" => self.settings.airspace.obstacle = value,

                    "max_level" => self.settings.options.max_level = value.parse::<u16>().unwrap(),
                    "radio" => self.settings.options.radio = value == "yes",
                    "north" => self.settings.options.north = value.parse::<f64>().unwrap(),
                    "south" => self.settings.options.south = value.parse::<f64>().unwrap(),
                    "format" => self.settings.options.format = match value.as_ref() {
                        "ratonly" => Format::RatOnly,
                        "competition" => Format::Competition,
                        _ => Format::OpenAir,
                    },
                    _ => (),
                }
                true
            }
            Msg::ExtraClear(category) => {
                match category {
                    ExtraCategory::Rat => self.settings.rat.clear(),
                    ExtraCategory::Loa => self.settings.loa.clear(),
                    ExtraCategory::Wave => self.settings.wave.clear(),
                }
                true
            }
            Msg::ExtraSet(setting) => {
                let set = match setting.category {
                    ExtraCategory::Rat => &mut self.settings.rat,
                    ExtraCategory::Loa => &mut self.settings.loa,
                    ExtraCategory::Wave => &mut self.settings.wave,
                };

                if setting.checked {
                    set.replace(setting.name);
                } else {
                    set.remove(&setting.name);
                }
                true
            }
            Msg::Save => {
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

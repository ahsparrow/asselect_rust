use gloo_file::{Blob, ObjectUrl};
use gloo_storage::{LocalStorage, Storage};
use gloo_utils::document;
use wasm_bindgen::JsCast;
use yew::{
    function_component, html, use_effect_with_deps, use_reducer, use_state,
    Callback,
};

use components::{AirspaceTab, ExtraPanel, ExtraTab, NotamTab, OptionsTab, Tabs};
use state::{Action, State};
use yaixm::convert::openair;
use yaixm::util::{fetch_yaixm, gliding_sites, loa_names, rat_names, wav_names};

mod components;
mod state;
mod yaixm;

// Callback data structures
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

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State{ settings: LocalStorage::get("settings").unwrap_or_default()});
    let yaixm = use_state(|| None);

    // Fetch YAIXM data
    {
        let yaixm = yaixm.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let data = fetch_yaixm().await;
                    yaixm.set(data.ok());
                });
                || ()
            },
            (),
        );
    }

    // Airspace settings callback
    let onairspace_set = {
        let state = state.clone();
        Callback::from(move |setting: AirspaceSetting| {
            state.dispatch(Action::Set {
                name: setting.name,
                value: setting.value,
            })
        })
    };

    // RAT/LOA/Wave setting callback
    let onextra_set = {
        let state = state.clone();
        Callback::from(move |setting: ExtraSetting| match setting.category {
            ExtraCategory::Rat => state.dispatch(Action::SetRat {
                name: setting.name,
                checked: setting.checked,
            }),
            ExtraCategory::Loa => state.dispatch(Action::SetLoa {
                name: setting.name,
                checked: setting.checked,
            }),
            ExtraCategory::Wave => state.dispatch(Action::SetWave {
                name: setting.name,
                checked: setting.checked,
            }),
        })
    };

    // RAT/LOA/Wave clear callback
    let onextra_clear = {
        let state = state.clone();
        Callback::from(move |category: ExtraCategory| match category {
            ExtraCategory::Rat => state.dispatch(Action::ClearRat),
            ExtraCategory::Loa => state.dispatch(Action::ClearLoa),
            ExtraCategory::Wave => state.dispatch(Action::ClearWave),
        })
    };

    // Airspace file save callback
    let onsave = {
        let yaixm = yaixm.clone();
        let state = state.clone();
        Callback::from(move |_| {
            // Save settings in local storage
            LocalStorage::set("settings", &state.settings).ok();

            // Create OpenAir data
            let oa = openair(yaixm.as_ref().unwrap(), &state.settings);
            let blob = Blob::new(oa.as_str());
            let object_url = ObjectUrl::from(blob);

            // Trigger a "fake" download to save the data
            let download_anchor = document()
                .get_element_by_id("download")
                .expect("No document")
                .dyn_into::<web_sys::HtmlAnchorElement>()
                .expect("No anchor element");

            download_anchor.set_href(&object_url);
            download_anchor.click();
        })
    };

    // HTML rendering
    let html_logic = match yaixm.as_ref() {
        // Render full interface if YAIXM data is available
        Some(yaixm) => {
            let airspace_settings = state.settings.airspace.clone();
            let airspace_options = state.settings.options.clone();

            let mut gliding_sites = gliding_sites(yaixm);
            gliding_sites.sort();

            let rat_selected = state.settings.rat.clone();
            let rat_names = rat_names(yaixm);

            let loa_selected = state.settings.loa.clone();
            let loa_names = loa_names(yaixm);

            let wav_selected = state.settings.wave.clone();
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
                    <AirspaceTab settings={airspace_settings} {gliding_sites} callback={onairspace_set.clone()} />
                    <OptionsTab options={airspace_options} callback={onairspace_set.clone()} />
                    <ExtraTab names={extra_names} categories={vec![ExtraCategory::Rat, ExtraCategory::Loa, ExtraCategory::Wave]} on_clear={onextra_clear.clone()}>
                      <ExtraPanel category={ExtraCategory::Rat} names={rat_names} selected={rat_selected} callback={onextra_set.clone()}/>
                      <ExtraPanel category={ExtraCategory::Loa} names={loa_names} selected={loa_selected} callback={onextra_set.clone()}/>
                      <ExtraPanel category={ExtraCategory::Wave} names={wav_names} selected={wav_selected} callback={onextra_set.clone()}/>
                    </ExtraTab>
                    <NotamTab />
                  </Tabs>
                </div>
                <div class="container">
                  <div class="ml-4 mt-4">
                    <button class="button is-primary" onclick={onsave}>
                      {"Save"}
                    </button>
                    <a id="download" hidden=true download="openair.txt">{"Download"}</a>
                  </div>
                </div>
                </>
            }
        }
        // Waiting for YAIXM data
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
    };

    html! {
        {html_logic}
    }
}

fn main() {
    //wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

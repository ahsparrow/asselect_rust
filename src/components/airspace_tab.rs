use crate::{Airspace, AirspaceSetting};
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub settings: Airspace,
    pub gliding_sites: Vec<String>,
    pub callback: Callback<AirspaceSetting>,
}

#[function_component(AirspaceTab)]
pub fn airspace_tab(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let id = e.target_unchecked_into::<HtmlInputElement>().id();
        let value = e.target_unchecked_into::<HtmlInputElement>().value();

        AirspaceSetting {
            id,
            value,
        }
    });

    let set = &props.settings;

    let gliding_sites = || {
        props.gliding_sites
            .iter()
            .map(|name| {
                html! {
                    <option selected={*name==set.home} >{name}</option>
                }
            })
            .collect::<Html>()
        };

    html! {
        <div>
          <div class="columns">

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"ATZ:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="atz" onchange={onchange.clone()}>
                      <option value="classd" selected={set.atz == "classd"}>{"Class D"}</option>
                      <option value="ctr" selected={set.atz == "ctr"}>{"Control Zone"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"ILS Feather:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="ils" onchange={onchange.clone()}>
                      <option value="atz" selected={set.ils == "atz"}>{"As ATZ"}</option>
                      <option value="classf" selected={set.ils == "classf"}>{"Class F"}</option>
                      <option value="classg" selected={set.ils == "classg"}>{"Class G"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="columns">

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Unlicensed Airfield:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="unlicensed" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.unlicensed == "exclude"}>{"Exclude"}</option>
                      <option value="classf" selected={set.unlicensed == "classf"}>{"Class F"}</option>
                      <option value="classg" selected={set.unlicensed == "classg"}>{"Class G"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Microlight Airfield:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="microlight" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.microlight == "exclude"}>{"Exclude"}</option>
                      <option value="classf" selected={set.microlight == "classf"}>{"Class F"}</option>
                      <option value="classg" selected={set.microlight == "classg"}>{"Class G"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="columns">

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Gliding Airfield:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="gliding" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.unlicensed == "exclude"}>{"Exclude"}</option>
                      <option value="glidingsector" selected={set.unlicensed == "glidingsector"}>{"Gliding Sector"}</option>
                      <option value="classf" selected={set.unlicensed == "classf"}>{"Class F"}</option>
                      <option value="classg" selected={set.unlicensed == "classg"}>{"Class G"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Exclude Home Airfield:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="home" onchange={onchange.clone()}>
                      <option selected={set.home=="None"}>{"None"}</option>
                      { gliding_sites() }
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="columns">

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"HIRTA/GVS:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="hirta_gvs" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.unlicensed == "exclude"}>{"Exclude"}</option>
                      <option value="danger" selected={set.unlicensed == "danger"}>{"Danger"}</option>
                      <option value="restricted" selected={set.unlicensed == "restricted"}>{"Restricted"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Obstacle:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="obstacle" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.unlicensed == "exclude"}>{"Exclude"}</option>
                      <option value="include" selected={set.unlicensed == "include"}>{"Include"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>

        </div>
    }
}

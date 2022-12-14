use crate::state::{Airspace, AirType, Format, Options};
use crate::AirspaceSetting;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub settings: Airspace,
    pub options: Options,
    pub callback: Callback<AirspaceSetting>,
}

#[function_component(OptionsTab)]
pub fn options_tab(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let name = e.target_unchecked_into::<HtmlInputElement>().name();
        let value = e.target_unchecked_into::<HtmlInputElement>().value();

        AirspaceSetting { name, value }
    });

    let set = &props.settings;
    let opts = &props.options;

    html! {
        <div class="box">
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Format:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select name="format" onchange={onchange.clone()}>
                      <option value="openair" selected={opts.format == Format::OpenAir}>{"OpenAir"}</option>
                      <option value="ratonly" selected={opts.format == Format::RatOnly}>{"RA(T) only"}</option>
                      <option value="competition" selected={opts.format == Format::Competition}>{"Competition"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Maximum Level:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select name="max_level" onchange={onchange.clone()}>
                      <option value="600" selected={opts.max_level == 600}>{"Unlimited"}</option>
                      <option value="195" selected={opts.max_level == 195}>{"FL195"}</option>
                      <option value="125" selected={opts.max_level == 125}>{"FL125"}</option>
                      <option value="105" selected={opts.max_level == 105}>{"FL105"}</option>
                      <option value="65" selected={opts.max_level == 65}>{"FL65"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

          </div>

          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Omit North of:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select name="north" onchange={onchange.clone()}>
                      <option value="59.0" selected={(opts.north - 59.0).abs() < 0.1}>{"None"}</option>
                      <option value="54.9" selected={(opts.north - 54.9).abs() < 0.1}>{"Carlisle"}</option>
                      <option value="53.7" selected={(opts.north - 53.7).abs() < 0.1}>{"Hull"}</option>
                      <option value="52.9" selected={(opts.north - 52.9).abs() < 0.1}>{"Nottingham"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Omit South of:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select name="south" onchange={onchange.clone()}>
                      <option value="49.0" selected={(opts.south - 49.0).abs() < 0.1}>{"None"}</option>
                      <option value="51.8" selected={(opts.south - 51.8).abs() < 0.1}>{"Oxford"}</option>
                      <option value="52.9" selected={(opts.south - 52.9).abs() < 0.1}>{"Nottingham"}</option>
                      <option value="53.7" selected={(opts.south - 53.7).abs() < 0.1}>{"Hull"}</option>
                      <option value="54.9" selected={(opts.south - 54.9).abs() < 0.1}>{"Carlisle"}</option>
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
                    <select name="hirta_gvs" onchange={onchange.clone()}>
                      <option value="exclude" selected={set.hirta_gvs.is_none()}>{"Exclude"}</option>
                      <option value="danger" selected={set.hirta_gvs == Some(AirType::Danger)}>{"Danger"}</option>
                      <option value="restricted" selected={set.hirta_gvs == Some(AirType::Restricted)}>{"Restricted"}</option>
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
                    <select name="obstacle" onchange={onchange.clone()}>
                      <option value="exclude" selected={!set.obstacle}>{"Exclude"}</option>
                      <option value="include" selected={set.obstacle}>{"Include"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
    }
}

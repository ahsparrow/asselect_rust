use crate::{Options, AirspaceSetting};
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Event, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub options: Options,
    pub callback: Callback<AirspaceSetting>
}

#[function_component(OptionsTab)]
pub fn options_tab(props: &Props) -> Html {
    let onchange = props.callback.reform(|e: Event| {
        let id = e.target_unchecked_into::<HtmlInputElement>().id();
        let value = e.target_unchecked_into::<HtmlInputElement>().value();

        AirspaceSetting {
            id,
            value,
        }
    });

    let opts = &props.options;

    html! {
        <div>
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Maximum Level:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="max_level" onchange={onchange.clone()}>
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

            <div class="column is-one-third">
              <div class="field">
                <label class="label is-small">{"Append Radio Frequencies:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="radio" onchange={onchange.clone()}>
                      <option value="no" selected={!opts.radio}>{"No"}</option>
                      <option value="yes" selected={opts.radio}>{"Yes"}</option>
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
                    <select id="north" onchange={onchange.clone()}>
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
                    <select id="south" onchange={onchange.clone()}>
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
                <label class="label is-small">{"Format:"}</label>
                <div class="control">
                  <div class="select is-fullwidth">
                    <select id="format" onchange={onchange.clone()}>
                      <option value="openair" selected={opts.format == "openair"}>{"OpenAir"}</option>
                      <option value="ratonly" selected={opts.format == "ratonly"}>{"RA(T) only"}</option>
                      <option value="competition" selected={opts.format == "competition"}>{"Competition"}</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
    }
}

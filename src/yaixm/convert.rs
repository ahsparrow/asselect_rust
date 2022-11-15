use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::yaixm;

// Settings
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Airspace {
    pub atz: String,
    pub ils: String,
    pub unlicensed: String,
    pub microlight: String,
    pub gliding: String,
    pub home: String,
    pub hirta_gvs: String,
    pub obstacle: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Options {
    pub max_level: u16,
    pub radio: bool,
    pub north: f64,
    pub south: f64,
    pub format: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub airspace: Airspace,
    pub options: Options,
    pub loa: HashSet<String>,
    pub rat: HashSet<String>,
    pub wav: HashSet<String>,
}

// Setting defaults
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
            obstacle: "exclude".to_string(),
        }
    }
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

fn do_name(feature: &yaixm::Feature, _vol: &yaixm::Volume, _settings: &Settings) -> String {
    format!("AN {}", feature.name)
}

pub fn openair(yaixm: &yaixm::Yaixm, settings: &Settings) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let airspace = &yaixm.airspace;

    for feature in airspace {
        for vol in &feature.geometry {
            output.push("*".to_string());
            //output.push(do_type(feature, vol, settings));
            output.push(do_name(feature, vol, settings));
        }
    }

    output
}

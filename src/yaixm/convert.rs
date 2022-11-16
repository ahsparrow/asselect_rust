use crate::yaixm::{
    local_type_str, rule_str, Feature, IcaoType, LocalType, Rule, Service, Volume, Yaixm,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Settings
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

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
    pub format: Format,
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
            format: Format::OpenAir,
        }
    }
}

fn do_name(feature: &Feature, vol: &Volume, n: usize, settings: &Settings) -> String {
    let name = if let Some(name) = &vol.name {
        name.clone()
    } else {
        let mut name = feature.name.clone();

        let mut rules = feature.rules.clone().unwrap_or_default();
        rules.extend(vol.rules.clone().unwrap_or_default());

        // Base type name
        if let Some(LocalType::NoAtz) | Some(LocalType::Ul) = feature.local_type {
            name += " A/F"
        } else if let Some(LocalType::Matz)
        | Some(LocalType::Dz)
        | Some(LocalType::Gvs)
        | Some(LocalType::Hirta)
        | Some(LocalType::Ils)
        | Some(LocalType::Laser) = feature.local_type
        {
            name.push(' ');
            name += local_type_str(feature.local_type.as_ref().unwrap());
        } else if feature.icao_type == IcaoType::Atz {
            name += " ATZ";
        } else if rules.contains(&Rule::Raz) {
            name += " RAZ";
        }

        // Optional sequence number
        if settings.options.format == Format::Competition && feature.geometry.len() > 1 {
            name.push('-');
            if let Some(seqno) = vol.seqno {
                name += &seqno.to_string();

                if let Some(subseq) = vol.subseq {
                    name.push(subseq);
                }
            } else {
                let x = (b'A'..=b'Z').map(|c| c as char).nth(n);
                name.push(x.unwrap());
            }
        }

        // SI & NOTAM qualifiers
        let mut qualifiers = rules
            .iter()
            .filter(|&x| x == &Rule::Si || x == &Rule::Notam)
            .map(rule_str)
            .collect::<Vec<&str>>();

        if !qualifiers.is_empty() {
            qualifiers.sort();
            qualifiers.reverse();
            name.push(' ');
            name += &qualifiers.join("/");
        }

        // Optionally append frequency
        if settings.options.radio {
            if let Some(freq) = vol.frequency {
                name += format!(" {:.3}", freq).as_ref();
            }
        };

        name
    };

    format!("AN {}", name)
}

fn merge_services(airspace: &mut Vec<Feature>, services: &Vec<Service>) {
    // Create frequency map
    let mut freqs = HashMap::new();
    for service in services {
        for id in &service.controls {
            freqs.insert(id, service.frequency);
        }
    }

    // Add frequency properties
    for feature in airspace {
        for volume in &mut feature.geometry {
            let id = volume.id.as_ref().or(feature.id.as_ref());
            if let Some(id) = id {
                if let Some(f) = freqs.get(&id) {
                    volume.frequency = Some(*f);
                }
            }
        }
    }
}

pub fn openair(yaixm: &Yaixm, settings: &Settings) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let mut airspace = yaixm.airspace.clone();

    merge_services(&mut airspace, &yaixm.service);

    for feature in airspace {
        for (n, vol) in feature.geometry.iter().enumerate() {
            output.push("*".to_string());
            //output.push(do_type(feature, vol, settings));
            output.push(do_name(&feature, vol, n, settings));
        }
    }

    output
}

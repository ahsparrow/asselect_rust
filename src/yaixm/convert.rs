use crate::yaixm::util::{format_distance, format_latlon, format_level, norm_level};
use crate::yaixm::{
    icao_class_str, local_type_str, rule_str, Arc, Boundary, Circle, Feature, IcaoType, LocalType,
    Rule, Service, Volume, Yaixm,
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
    pub wave: HashSet<String>,
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

fn airfilter(feature: &Feature, vol: &Volume, settings: &Settings) -> bool {
    let exclude = match feature.local_type {
        // No-ATZ
        Some(LocalType::NoAtz) => &settings.airspace.unlicensed == "exclude",
        // Unlicensed
        Some(LocalType::Ul) => &settings.airspace.microlight == "exclude",
        // Wave Box
        Some(LocalType::Glider) if feature.icao_type == IcaoType::DOther => {
            !settings.wave.contains(&feature.name)
        }
        // Gliding Site
        Some(LocalType::Glider) => {
            settings.airspace.gliding == "exclude" || settings.airspace.home == feature.name
        }
        // HIRTA/GVS/Laser
        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
            settings.airspace.hirta_gvs == "exclude"
        }
        _ => false,
    };

    !(exclude || (norm_level(&vol.lower) >= settings.options.max_level))
}

fn do_name(feature: &Feature, vol: &Volume, n: usize, settings: &Settings) -> String {
    let name = if let Some(name) = &vol.name {
        name.clone()
    } else {
        let mut name = feature.name.clone();

        let rules = feature
            .rules
            .iter()
            .chain(vol.rules.iter())
            .flatten()
            .collect::<HashSet<&Rule>>();

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
            .into_iter()
            .filter(|&x| *x == Rule::Si || *x == Rule::Notam)
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

    format!("AN {}\n", name)
}

fn do_type(feature: &Feature, vol: &Volume, settings: &Settings) -> String {
    let atz = match settings.airspace.atz.as_str() {
        "classd" => "D",
        _ => "CTR",
    };

    let ils = match settings.airspace.ils.as_str() {
        "atz" => atz,
        "classf" => "F",
        _ => "G",
    };

    let noatz = match settings.airspace.unlicensed.as_str() {
        "classf" => "F",
        _ => "G",
    };

    let ul = match settings.airspace.microlight.as_str() {
        "classf" => "F",
        _ => "G",
    };

    let glider = match settings.airspace.gliding.as_str() {
        "classf" => "F",
        "classg" => "G",
        _ => "W",
    };

    let hirta_gvs = match settings.airspace.hirta_gvs.as_str() {
        "restricted" => "R",
        _ => "Q",
    };

    let rules = feature
        .rules
        .iter()
        .chain(vol.rules.iter())
        .flatten()
        .collect::<HashSet<&Rule>>();

    let comp = settings.options.format == Format::Competition;

    let openair_type = if rules.contains(&Rule::Notam) {
        // NOTAM activated airspace
        "G"
    } else {
        match feature.icao_type {
            IcaoType::Atz => atz,
            IcaoType::D => {
                if comp && rules.contains(&Rule::Si) {
                    // Danger area with SI
                    "P"
                } else {
                    // Danger area without SI
                    "Q"
                }
            }
            IcaoType::DOther => {
                if comp
                    && feature.local_type == Some(LocalType::Dz)
                    && rules.contains(&Rule::Intense)
                {
                    // Intense drop zone - competition
                    "P"
                } else {
                    match feature.local_type {
                        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
                            hirta_gvs
                        }
                        Some(LocalType::Glider) => "W",
                        _ => "Q",
                    }
                }
            }
            IcaoType::Other => match feature.local_type {
                Some(LocalType::Glider) => {
                    if rules.contains(&Rule::Loa) {
                        "W"
                    } else {
                        glider
                    }
                }
                Some(LocalType::Ils) => ils,
                Some(LocalType::Matz) => "MATZ",
                Some(LocalType::NoAtz) => noatz,
                Some(LocalType::Rat) => "P",
                Some(LocalType::Tmz) => "TMZ",
                Some(LocalType::Ul) => ul,
                Some(LocalType::Rmz) => "RMZ",
                _ => "OTHER",
            },
            IcaoType::P => "P",
            IcaoType::R => "R",
            _ => {
                if rules.contains(&Rule::Tmz) {
                    "TMZ"
                } else if rules.contains(&Rule::Rmz) {
                    "RMZ"
                } else {
                    match &vol.icao_class {
                        Some(vc) => icao_class_str(vc),
                        None => match &feature.icao_class {
                            Some(fc) => icao_class_str(fc),
                            None => "OTHER",
                        },
                    }
                }
            }
        }
    };

    format!("AC {}\n", openair_type)
}

fn do_levels(volume: &Volume) -> String {
    format!(
        "AL {}\nAH {}\n",
        &format_level(&volume.lower),
        &format_level(&volume.upper)
    )
}

fn do_point(point: &str) -> String {
    format!("DP {}\n", format_latlon(point))
}

fn do_line(line: &[String]) -> String {
    line.iter()
        .map(|x| do_point(x))
        .collect::<Vec<String>>()
        .join("")
}

fn do_circle(circle: &Circle) -> String {
    format!(
        "V X={}\nDC {}\n",
        format_latlon(&circle.centre),
        format_distance(&circle.radius)
    )
}

fn do_arc(arc: &Arc, from: &str) -> String {
    let dir = if arc.dir == "cw" {
        "+"
    } else {
        "-"
    };

    format!(
        "V D={}\nV X={}\nDB {}, {}\n",
        dir,
        format_latlon(&arc.centre),
        format_latlon(from),
        format_latlon(&arc.to)
    )
}

fn do_boundary(boundary: &[Boundary]) -> String {
    let mut out = String::new();
    let mut prev = "";

    for segment in boundary {
        match segment {
            Boundary::Line(line) => {
                out.push_str(&do_line(line));
                prev = line.last().unwrap();
            }
            Boundary::Arc(arc) => {
                out.push_str(&do_arc(arc, prev));
                prev = &arc.to;
            }
            Boundary::Circle(circle) => out.push_str(&do_circle(circle)),
        }
    }

    // Close the polygon
    if let Boundary::Line(line) = &boundary[0] {
        if line[0] != prev {
            out.push_str(&do_point(&line[0]));
        }
    }

    out
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

pub fn openair(yaixm: &Yaixm, settings: &Settings) -> String {
    let mut output = String::new();
    let mut airspace = yaixm.airspace.clone();

    merge_services(&mut airspace, &yaixm.service);

    for feature in airspace {
        for (n, vol) in feature.geometry.iter().enumerate() {
            if airfilter(&feature, vol, settings) {
                output.push_str("*\n");
                output.push_str(&do_type(&feature, vol, settings));
                output.push_str(&do_name(&feature, vol, n, settings));
                output.push_str(&do_levels(vol));
                output.push_str(&do_boundary(&vol.boundary));
            }
        }
    }
    output
}

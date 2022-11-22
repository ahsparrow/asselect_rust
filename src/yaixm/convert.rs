use crate::yaixm::util::{format_distance, format_latlon, format_level, norm_level};
use crate::yaixm::{
    Arc, Boundary, Circle, Feature, IcaoClass, IcaoType, LocalType, Rule, Service,
    Volume, Yaixm,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

// Settings
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AirType {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    P,
    Q,
    R,
    W,
    Cta,
    Ctr,
    Matz,
    Other,
    Rmz,
    Tmz,
}

impl fmt::Display for AirType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AirType::A => write!(f, "A"),
            AirType::B => write!(f, "B"),
            AirType::C => write!(f, "C"),
            AirType::D => write!(f, "D"),
            AirType::E => write!(f, "E"),
            AirType::F => write!(f, "F"),
            AirType::G => write!(f, "G"),
            AirType::P => write!(f, "P"),
            AirType::Q => write!(f, "Q"),
            AirType::R => write!(f, "R"),
            AirType::W => write!(f, "W"),
            AirType::Cta => write!(f, "CTA"),
            AirType::Ctr => write!(f, "CTR"),
            AirType::Matz => write!(f, "MATZ"),
            AirType::Other => write!(f, "OTHER"),
            AirType::Rmz => write!(f, "RMZ"),
            AirType::Tmz => write!(f, "RMZ"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Airspace {
    pub atz: AirType,
    pub ils: Option<AirType>,
    pub unlicensed: Option<AirType>,
    pub microlight: Option<AirType>,
    pub gliding: Option<AirType>,
    pub home: Option<String>,
    pub hirta_gvs: Option<AirType>,
    pub obstacle: bool,
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
            atz: AirType::Ctr,
            ils: None,
            unlicensed: None,
            microlight: None,
            gliding: None,
            home: None,
            hirta_gvs: None,
            obstacle: false,
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

// Remove unwanted feature/volume
fn airfilter(feature: &Feature, vol: &Volume, settings: &Settings) -> bool {
    let exclude = match feature.local_type {
        // No-ATZ
        Some(LocalType::NoAtz) => settings.airspace.unlicensed == None,
        // Microlight
        Some(LocalType::Ul) => settings.airspace.microlight == None,
        // Wave Box
        Some(LocalType::Glider) if feature.icao_type == IcaoType::DOther => {
            !settings.wave.contains(&feature.name)
        }
        // Gliding Site
        Some(LocalType::Glider) => {
            settings.airspace.gliding == None
                || settings.airspace.home.as_ref() == Some(&feature.name)
        }
        // HIRTA/GVS/Laser
        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
            settings.airspace.hirta_gvs == None
        }
        _ => false,
    };

    !(exclude || (norm_level(&vol.lower) >= settings.options.max_level))
}

// Give each volume a name
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
            name += feature.local_type.unwrap().to_string().as_str();
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
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        if !qualifiers.is_empty() {
            qualifiers.sort();
            qualifiers.reverse();
            name.push(' ');
            name += format!("({})", qualifiers.join("/")).as_ref();
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

// Give each volume a type
fn do_type(feature: &Feature, volume: &Volume, settings: &Settings) -> String {
    let atz = settings.airspace.atz.to_string();
    let ul = settings
        .airspace
        .microlight
        .unwrap_or(AirType::Other)
        .to_string();
    let ils = settings
        .airspace
        .ils
        .unwrap_or(settings.airspace.atz)
        .to_string();
    let noatz = settings
        .airspace
        .unlicensed
        .unwrap_or(AirType::Other)
        .to_string();
    let gliding = settings
        .airspace
        .gliding
        .unwrap_or(AirType::Other)
        .to_string();
    let hirta_gvs = settings
        .airspace
        .hirta_gvs
        .unwrap_or(AirType::Other)
        .to_string();

    let rules = feature
        .rules
        .iter()
        .chain(volume.rules.iter())
        .flatten()
        .collect::<HashSet<&Rule>>();

    let comp = settings.options.format == Format::Competition;

    let volume_class = volume
        .icao_class
        .or(feature.icao_class)
        .unwrap_or(IcaoClass::G)
        .to_string();

    let openair_type = if rules.contains(&Rule::Notam) {
        // NOTAM activated airspace
        "G"
    } else {
        match feature.icao_type {
            IcaoType::Atz => atz.as_str(),
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
                            hirta_gvs.as_str()
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
                        gliding.as_str()
                    }
                }
                Some(LocalType::Ils) => ils.as_str(),
                Some(LocalType::Matz) => "MATZ",
                Some(LocalType::NoAtz) => noatz.as_str(),
                Some(LocalType::Rat) => "P",
                Some(LocalType::Tmz) => "TMZ",
                Some(LocalType::Ul) => ul.as_str(),
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
                    volume_class.as_str()
                }
            }
        }
    };

    format!("AC {}\n", openair_type)
}

fn do_levels(volume: &Volume) -> String {
    format!(
        "AL {}\nAH {}\n",
        format_level(&volume.lower),
        format_level(&volume.upper)
    )
}

fn do_freq(freq: f64) -> String {
    format!("AF {:.3}\n", freq)
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
    let dir = if arc.dir == "cw" { "+" } else { "-" };

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
    let mut frequencies = HashMap::new();
    for service in services {
        for id in &service.controls {
            frequencies.insert(id, service.frequency);
        }
    }

    // Add frequency properties
    for feature in airspace {
        for volume in &mut feature.geometry {
            let volume_freq = if let Some(id) = &volume.id {
                frequencies.get(&id)
            } else {
                None
            };

            let feature_freq = if let Some(id) = &feature.id {
                frequencies.get(&id)
            } else {
                None
            };

            volume.frequency = volume_freq.or(feature_freq).cloned();
        }
    }
}

pub fn openair(yaixm: &Yaixm, settings: &Settings) -> String {
    let mut output = String::new();
    let mut airspace = yaixm.airspace.clone();

    merge_services(&mut airspace, &yaixm.service);

    for feature in airspace {
        for (n, volume) in feature.geometry.iter().enumerate() {
            if airfilter(&feature, volume, settings) {
                output.push_str("*\n");
                output.push_str(&do_type(&feature, volume, settings));
                output.push_str(&do_name(&feature, volume, n, settings));
                if let Some(freq) = volume.frequency {
                    output.push_str(&do_freq(freq));
                }
                output.push_str(&do_levels(volume));
                output.push_str(&do_boundary(&volume.boundary));
            }
        }
    }
    output
}

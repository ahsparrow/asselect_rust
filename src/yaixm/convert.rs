use crate::state::{AirType, Format, Settings};
use crate::yaixm::{
    Arc, Boundary, Circle, Feature, IcaoClass, IcaoType, Loa, LocalType, Rule, Service, Volume, Yaixm,
};
use std::collections::{HashMap, HashSet};

impl IcaoClass {
    fn as_str(&self) -> &'static str {
        match self {
            IcaoClass::A => "A",
            IcaoClass::B => "B",
            IcaoClass::C => "C",
            IcaoClass::D => "D",
            IcaoClass::E => "E",
            IcaoClass::F => "F",
            IcaoClass::G => "G",
        }
    }
}

impl LocalType {
    fn as_str(&self) -> &'static str {
        match self {
            LocalType::Dz => "DZ",
            LocalType::Glider => "GLIDER",
            LocalType::Gvs => "GVS",
            LocalType::Hirta => "HIRTA",
            LocalType::Ils => "ILS",
            LocalType::Laser => "LASER",
            LocalType::Matz => "MATZ",
            LocalType::NoAtz => "NOATZ",
            LocalType::Rat => "RAT",
            LocalType::Rmz => "RMZ",
            LocalType::Ul => "UL",
            LocalType::Tmz => "TMZ",
        }
    }
}

impl Rule {
    fn as_str(&self) -> &'static str {
        match self {
            Rule::Intense => "INTENSE",
            Rule::Loa => "LOA",
            Rule::NoSsr => "NOSSR",
            Rule::Notam => "NOTAM",
            Rule::Raz => "RAZ",
            Rule::Rmz => "RMZ",
            Rule::Si => "SI",
            Rule::Tra => "TRA",
            Rule::Tmz => "TMZ",
        }
    }
}

impl AirType {
    fn as_str(&self) -> &'static str {
        match self {
            AirType::ClassA => "A",
            AirType::ClassB => "B",
            AirType::ClassC => "C",
            AirType::ClassD => "D",
            AirType::ClassE => "E",
            AirType::ClassF => "F",
            AirType::ClassG => "G",
            AirType::Prohibited => "P",
            AirType::Danger => "Q",
            AirType::Restricted => "R",
            AirType::Gliding => "W",
            AirType::Cta => "CTA",
            AirType::Ctr => "CTR",
            AirType::Matz => "MATZ",
            AirType::Other => "OTHER",
            AirType::Rmz => "RMZ",
            AirType::Tmz => "RMZ",
        }
    }
}

// Normalise all levels to flight level
fn norm_level(value: &str) -> u16 {
    if let Some(fl) = value.strip_prefix("FL") {
        fl.parse().unwrap()
    } else if value.ends_with(" ft") {
        value.split(' ').next().unwrap().parse::<u16>().unwrap() / 100
    } else {
        0
    }
}

// Openair level format
fn format_level(level: &str) -> String {
    if let Some(alt) = level.strip_suffix(" ft") {
        // Altitude
        alt.to_string() + "ALT"
    } else {
        // Flight level or SFC
        level.to_string()
    }
}

// Openair lat/lon format
fn format_latlon(latlon: &str) -> String {
    format!(
        "{}:{}:{} {} {}:{}:{} {}",
        &latlon[..2],
        &latlon[2..4],
        &latlon[4..6],
        &latlon[6..7],
        &latlon[8..11],
        &latlon[11..13],
        &latlon[13..15],
        &latlon[15..16]
    )
}

// Openair distance format
fn format_distance(distance: &str) -> String {
    match distance.split_once(' ') {
        Some((dist, unit)) => {
            if unit == "km" {
                format!("{:.3}", dist.parse::<f32>().unwrap() / 1.852)
            } else {
                dist.to_string()
            }
        }
        _ => "".to_string(),
    }
}
// Remove unwanted feature/volume
fn airfilter(feature: &Feature, vol: &Volume, settings: &Settings) -> bool {
    let exclude = match feature.local_type {
        // No-ATZ
        Some(LocalType::NoAtz) => settings.airspace.unlicensed.is_none(),
        // Microlight
        Some(LocalType::Ul) => settings.airspace.microlight.is_none(),
        // Wave Box
        Some(LocalType::Glider) if feature.icao_type == IcaoType::DOther => {
            !settings.wave.contains(&feature.name)
        }
        // Gliding Site
        Some(LocalType::Glider) => {
            settings.airspace.gliding.is_none()
                || settings.airspace.home.as_ref() == Some(&feature.name)
        }
        // HIRTA/GVS/Laser
        Some(LocalType::Hirta) | Some(LocalType::Gvs) | Some(LocalType::Laser) => {
            settings.airspace.hirta_gvs.is_none()
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
            name += feature.local_type.unwrap().as_str();
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
            .map(|x| x.as_str().to_string())
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
    let rules = feature
        .rules
        .iter()
        .chain(volume.rules.iter())
        .flatten()
        .collect::<HashSet<&Rule>>();

    let comp = settings.options.format == Format::Competition;

    let openair_type = if rules.contains(&Rule::Notam) {
        // NOTAM activated airspace
        "G"
    } else {
        match feature.icao_type {
            IcaoType::Atz => settings.airspace.atz.as_str(),
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
                            settings
                                .airspace
                                .hirta_gvs
                                .unwrap_or(AirType::Other)
                                .as_str()
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
                        settings.airspace.gliding.unwrap_or(AirType::Other).as_str()
                    }
                }
                Some(LocalType::Ils) => settings
                    .airspace
                    .ils
                    .unwrap_or(settings.airspace.atz)
                    .as_str(),
                Some(LocalType::Matz) => "MATZ",
                Some(LocalType::NoAtz) => settings
                    .airspace
                    .unlicensed
                    .unwrap_or(AirType::Other)
                    .as_str(),
                Some(LocalType::Rat) => "P",
                Some(LocalType::Tmz) => "TMZ",
                Some(LocalType::Ul) => settings
                    .airspace
                    .microlight
                    .unwrap_or(AirType::Other)
                    .as_str(),
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
                    volume
                        .icao_class
                        .or(feature.icao_class)
                        .unwrap_or(IcaoClass::G)
                        .as_str()
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

// Search for volume id and return indices of feature/volume
fn find_volume(airspace: &[Feature], volume_id: &str) -> Option<(usize, usize)> {
    for (f, feature) in airspace.iter().enumerate() {
        for (v, volume) in feature.geometry.iter().enumerate() {
            if volume.id.as_deref() == Some(volume_id) {
                return Some((f, v));
            }
        }
    }
    None
}

fn merge_loa(airspace: &mut Vec<Feature>, loas: &Vec<&Loa>) {
    // Add new features
    for loa in loas {
        for area in &loa.areas {
            for feature in &area.add {
                let mut feature = feature.clone();
                let mut rules = feature.rules.unwrap_or_default();

                // Add LOA rule
                rules.replace(Rule::Loa);
                feature.rules = Some(rules);

                airspace.push(feature);
            }
        }
    }

    // Replace volumes
    for loa in loas {
        for area in &loa.areas {
            if let Some(replacements) = &area.replace {
                for replace in replacements {
                    // Find replacement volume
                    if let Some((f, v)) = find_volume(airspace, &replace.id) {
                        let r = (*replace).clone();

                        // Update seqno from existing volume and add to feature
                        if let Some(seqno) = airspace[f].geometry[v].seqno {
                            for mut vol in r.geometry {
                                vol.seqno = Some(seqno);
                                airspace[f].geometry.push(vol);
                            }
                        }

                        // Delete the exiting volume
                        airspace[f].geometry.remove(v);

                        // Remove feature if no remaining geometry
                        if airspace[f].geometry.is_empty() {
                            airspace.remove(f);
                        }
                    }
                }
            }
        }
    }
}

pub fn openair(yaixm: &Yaixm, settings: &Settings) -> String {
    let mut output = String::new();
    let mut airspace = yaixm.airspace.clone();

    let loas = yaixm
        .loa
        .iter()
        .filter(|&x| (x.default == Some(true)) | settings.loa.contains(&x.name))
        .collect::<Vec<&Loa>>();
    merge_loa(&mut airspace, &loas);

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

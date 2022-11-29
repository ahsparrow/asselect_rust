use serde::Deserialize;
use std::collections::HashSet;

pub mod convert;
pub mod util;

#[derive(Clone, Copy, Deserialize, Debug, Eq, PartialEq)]
pub enum IcaoClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Debug)]
pub enum IcaoType {
    #[serde(rename = "ATZ")]
    Atz,
    #[serde(rename = "AWY")]
    Awy,
    #[serde(rename = "CTA")]
    Cta,
    #[serde(rename = "CTR")]
    Ctr,
    D,
    #[serde(rename = "D_OTHER")]
    DOther,
    #[serde(rename = "OTHER")]
    Other,
    P,
    R,
    #[serde(rename = "TMA")]
    Tma,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Debug)]
pub enum LocalType {
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "GLIDER")]
    Glider,
    #[serde(rename = "GVS")]
    Gvs,
    #[serde(rename = "HIRTA")]
    Hirta,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "LASER")]
    Laser,
    #[serde(rename = "MATZ")]
    Matz,
    #[serde(rename = "NOATZ")]
    NoAtz,
    #[serde(rename = "RAT")]
    Rat,
    #[serde(rename = "RMZ")]
    Rmz,
    #[serde(rename = "UL")]
    Ul,
    #[serde(rename = "TMZ")]
    Tmz,
}

#[derive(Clone, Copy, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum Rule {
    #[serde(rename = "INTENSE")]
    Intense,
    #[serde(rename = "LOA")]
    Loa,
    #[serde(rename = "NOSSR")]
    NoSsr,
    #[serde(rename = "NOTAM")]
    Notam,
    #[serde(rename = "RAZ")]
    Raz,
    #[serde(rename = "RMZ")]
    Rmz,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "TRA")]
    Tra,
    #[serde(rename = "TMZ")]
    Tmz,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Circle {
    pub centre: String,
    pub radius: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Arc {
    pub centre: String,
    pub dir: String,
    pub radius: String,
    pub to: String,
}

#[derive(Clone, Deserialize, Debug)]
pub enum Boundary {
    #[serde(rename = "circle")]
    Circle(Circle),
    #[serde(rename = "arc")]
    Arc(Arc),
    #[serde(rename = "line")]
    Line(Vec<String>),
}

#[derive(Clone, Deserialize, Debug)]
pub struct Volume {
    pub id: Option<String>,
    pub name: Option<String>,
    pub lower: String,
    pub upper: String,
    #[serde(rename = "class")]
    pub icao_class: Option<IcaoClass>,
    pub rules: Option<HashSet<Rule>>,
    pub seqno: Option<u8>,
    pub subseq: Option<char>,
    pub frequency: Option<f64>,
    pub boundary: Vec<Boundary>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Feature {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub icao_type: IcaoType,
    #[serde(rename = "localtype")]
    pub local_type: Option<LocalType>,
    #[serde(rename = "class")]
    pub icao_class: Option<IcaoClass>,
    pub rules: Option<HashSet<Rule>>,
    pub geometry: Vec<Volume>,
}

#[derive(Deserialize, Debug)]
pub struct Replace {
    pub id: String,
    pub geometry: Vec<Volume>,
}

#[derive(Deserialize, Debug)]
pub struct LoaArea {
    pub name: String,
    pub add: Option<Vec<Feature>>,
    pub replace: Option<Vec<Replace>>,
}

#[derive(Deserialize, Debug)]
pub struct Loa {
    pub name: String,
    pub default: Option<bool>,
    pub areas: Vec<LoaArea>,
}

#[derive(Deserialize, Debug)]
pub struct Obstacle {
    pub elevation: String,
    pub id: String,
    pub position: String,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    pub callsign: String,
    pub frequency: f64,
    pub controls: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub airac_date: String,
    pub timestamp: String,
    pub schema_version: u8,
    pub note: String,
    pub commit: String,
}

#[derive(Deserialize, Debug)]
pub struct Yaixm {
    pub airspace: Vec<Feature>,
    pub rat: Vec<Feature>,
    pub loa: Vec<Loa>,
    pub obstacle: Vec<Obstacle>,
    pub service: Vec<Service>,
    pub release: Release,
}

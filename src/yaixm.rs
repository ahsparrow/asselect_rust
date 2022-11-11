use serde::Deserialize;

pub mod openair;
pub mod util;

#[derive(Deserialize, Debug)]
pub enum IcaoClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub fn icao_class_str(class: &IcaoClass) -> &'static str {
    match class {
        IcaoClass::A => "A",
        IcaoClass::B => "B",
        IcaoClass::C => "C",
        IcaoClass::D => "D",
        IcaoClass::E => "D",
        IcaoClass::F => "F",
        IcaoClass::G => "G",
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[allow(nonstandard_style)]
pub enum IcaoType {
    ATZ,     // Air Traffic Zone
    AWY,     // Airway
    CTA,     // Control area. Upwards from a specified limit
    CTR,     // Control zone. Upwards from the surface
    D,       // Danger area
    D_OTHER, // Dangerous activity, but not a Danger area
    OTHER,   // As specified by localtype
    P,       // Prohibited area
    R,       // Restricted area
    TMA,     // Terminal control area
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum LocalType {
    DZ,     // Parachute drop zone
    GLIDER, // Gliding operations
    GVS,    // Gas venting station
    HIRTA,  // High intensity radio transmission area
    ILS,    // ILS feather
    LASER,  // Laser site
    MATZ,   // Military ATZ
    NOATZ,  // Non-ATZ airfield
    RAT,    // Temporary restricted area
    RMZ,    // Radio mandatory zone
    UL,     // Ultra-light strip
    TMZ,    // Transponder mandatory zone
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum Rule {
    INTENSE, // Intense activity
    LOA,     // Letter of agreement
    NOSSR,   // Non-SSR area
    NOTAM,   // NOTAM activated
    RAZ,     // Radio advisory zone
    RMZ,     // Radio mandatory zone
    SI,      // Statutary Instrument
    TRA,     // Temporary reserved area
    TMZ,     // Transponder mandatory zone
}

#[derive(Deserialize, Debug)]
pub struct Circle {
    pub centre: String,
    pub radius: String,
}

#[derive(Deserialize, Debug)]
pub struct Arc {
    pub centre: String,
    pub dir: String,
    pub radius: String,
    pub to: String,
}

#[derive(Deserialize, Debug)]
pub enum Boundary {
    #[serde(rename="circle")]
    Circle(Circle),
    #[serde(rename="arc")]
    Arc(Arc),
    #[serde(rename="line")]
    Line(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    pub lower: String,
    pub upper: String,
    #[serde(rename="class")]
    pub icao_class: Option<IcaoClass>,
    pub name: Option<String>,
    pub rules: Option<Vec<Rule>>,
    pub seqno: Option<u8>,
    pub boundary: Vec<Boundary>,
}

#[derive(Deserialize, Debug)]
pub struct Feature {
    pub name: String,
    #[serde(rename="type")]
    pub icao_type: IcaoType,
    #[serde(rename="localtype")]
    pub local_type: Option<LocalType>,
    #[serde(rename="class")]
    pub icao_class: Option<IcaoClass>,
    pub rules: Option<Vec<Rule>>,
    pub id: Option<String>,
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

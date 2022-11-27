use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::rc::Rc;
use yew::Reducible;

// Airspace types
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum AirType {
    ClassA,
    ClassB,
    ClassC,
    ClassD,
    ClassE,
    ClassF,
    ClassG,
    Danger,
    Cta,
    Ctr,
    Gliding,
    Matz,
    Other,
    Prohibited,
    Restricted,
    Rmz,
    Tmz,
}

// Output format
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    OpenAir,
    RatOnly,
    Competition,
}

// Airspace settings
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

// Additional options
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Options {
    pub max_level: u16,
    pub radio: bool,
    pub north: f64,
    pub south: f64,
    pub format: Format,
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

// Application settings
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Settings {
    pub airspace: Airspace,
    pub options: Options,
    pub loa: HashSet<String>,
    pub rat: HashSet<String>,
    pub wave: HashSet<String>,
}

// Application state
#[derive(Debug, Default, PartialEq)]
pub struct State {
    pub settings: Settings,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }
}

// State actions
pub enum Action {
    Set { name: String, value: String },
    SetLoa { name: String, checked: bool },
    SetRat { name: String, checked: bool },
    SetWave { name: String, checked: bool },
    ClearLoa,
    ClearRat,
    ClearWave,
}

impl Reducible for State {
    // Reducer Action type
    type Action = Action;

    // Reducer Function
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut settings = self.settings.clone();
        match action {
            Action::Set { name, value } => {
                match name.as_str() {
                    "unlicensed" => settings.airspace.unlicensed = default_set(value.as_str()),
                    "microlight" => settings.airspace.microlight = default_set(value.as_str()),
                    "gliding" => settings.airspace.gliding = default_set(value.as_str()),
                    "hirta_gvs" => settings.airspace.hirta_gvs = default_set(value.as_str()),
                    "obstacle" => settings.airspace.obstacle = value == "include",
                    "max_level" => settings.options.max_level = value.parse::<u16>().unwrap(),
                    "radio" => settings.options.radio = value == "yes",
                    "north" => settings.options.north = value.parse::<f64>().unwrap(),
                    "south" => settings.options.south = value.parse::<f64>().unwrap(),
                    "atz" => {
                        settings.airspace.atz = match value.as_str() {
                            "classd" => AirType::ClassD,
                            _ => AirType::Ctr,
                        }
                    }
                    "home" => {
                        settings.airspace.home = if value == "None" { None } else { Some(value) }
                    }
                    "format" => {
                        settings.options.format = match value.as_str() {
                            "ratonly" => Format::RatOnly,
                            "competition" => Format::Competition,
                            _ => Format::OpenAir,
                        }
                    }
                    _ => (),
                };
            }
            Action::SetLoa { name, checked } => {
                if checked {
                    settings.loa.replace(name);
                } else {
                    settings.loa.remove(&name);
                }
            }
            Action::SetRat { name, checked } => {
                if checked {
                    settings.rat.replace(name);
                } else {
                    settings.rat.remove(&name);
                }
            }
            Action::SetWave { name, checked } => {
                if checked {
                    settings.wave.replace(name);
                } else {
                    settings.wave.remove(&name);
                }
            }
            Action::ClearLoa => {
                settings.loa.clear();
            }
            Action::ClearRat => {
                settings.rat.clear();
            }
            Action::ClearWave => {
                settings.wave.clear();
            }
        }
        Self { settings }.into()
    }
}

// Helper function
fn default_set(value: &str) -> Option<AirType> {
    match value {
        "classf" => Some(AirType::ClassF),
        "classg" => Some(AirType::ClassG),
        "danger" => Some(AirType::Danger),
        "restricted" => Some(AirType::Restricted),
        "gsec" => Some(AirType::Gliding),
        _ => None,
    }
}

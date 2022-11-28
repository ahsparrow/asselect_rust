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
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut set = self.settings.clone();
        match action {
            // Set airspace option
            Action::Set { name, value } => {
                match name.as_str() {
                    "unlicensed" => set.airspace.unlicensed = default_set(value.as_str()),
                    "microlight" => set.airspace.microlight = default_set(value.as_str()),
                    "gliding" => set.airspace.gliding = default_set(value.as_str()),
                    "hirta_gvs" => set.airspace.hirta_gvs = default_set(value.as_str()),
                    "obstacle" => set.airspace.obstacle = value == "include",
                    "max_level" => set.options.max_level = value.parse::<u16>().unwrap(),
                    "radio" => set.options.radio = value == "yes",
                    "north" => set.options.north = value.parse::<f64>().unwrap(),
                    "south" => set.options.south = value.parse::<f64>().unwrap(),
                    "atz" => {
                        set.airspace.atz = match value.as_str() {
                            "classd" => AirType::ClassD,
                            _ => AirType::Ctr,
                        }
                    }
                    "home" => set.airspace.home = if value == "None" { None } else { Some(value) },
                    "format" => {
                        set.options.format = match value.as_str() {
                            "ratonly" => Format::RatOnly,
                            "competition" => Format::Competition,
                            _ => Format::OpenAir,
                        }
                    }
                    _ => (),
                };
            }
            // Include/exclude LOA
            Action::SetLoa { name, checked } => {
                if checked {
                    set.loa.replace(name);
                } else {
                    set.loa.remove(&name);
                }
            }
            // Include/exclude RAT
            Action::SetRat { name, checked } => {
                if checked {
                    set.rat.replace(name);
                } else {
                    set.rat.remove(&name);
                }
            }
            // Include/exclude wave box
            Action::SetWave { name, checked } => {
                if checked {
                    set.wave.replace(name);
                } else {
                    set.wave.remove(&name);
                }
            }
            // Clear all LOAs
            Action::ClearLoa => {
                set.loa.clear();
            }
            // Clear all RATs
            Action::ClearRat => {
                set.rat.clear();
            }
            // Clear all Wave boxes
            Action::ClearWave => {
                set.wave.clear();
            }
        }
        Self { settings: set }.into()
    }
}

// Default mapping value to airspace type
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

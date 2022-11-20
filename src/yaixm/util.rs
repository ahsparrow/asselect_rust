use crate::yaixm::{IcaoType, LocalType, Yaixm};
use gloo_net::http::Request;
use gloo_net::Error;

pub async fn fetch_yaixm() -> Result<Yaixm, Error> {
    let result = Request::get("yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await,
        Err(e) => Err(e),
    }
}

pub fn rat_names(yaixm: &Yaixm) -> Vec<String> {
    let rat = &yaixm.rat;
    rat.iter().map(|x| x.name.clone()).collect::<Vec<String>>()
}

pub fn loa_names(yaixm: &Yaixm) -> Vec<String> {
    let loa = &yaixm.loa;
    loa.iter()
        .filter(|x| !x.default.unwrap_or(false))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn wav_names(yaixm: &Yaixm) -> Vec<String> {
    yaixm
        .airspace
        .iter()
        .filter(|x| x.icao_type == IcaoType::DOther && x.local_type == Some(LocalType::Glider))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn gliding_sites(yaixm: &Yaixm) -> Vec<String> {
    yaixm
        .airspace
        .iter()
        .filter(|x| x.icao_type == IcaoType::Other && x.local_type == Some(LocalType::Glider))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn norm_level(value: &str) -> u16 {
    if let Some(fl) = value.strip_prefix("FL") {
        fl.parse().unwrap()
    } else if value.ends_with(" ft") {
        value.split(' ').next().unwrap().parse::<u16>().unwrap() / 100
    } else {
        0
    }
}

pub fn format_level(level: &str) -> String {
    if let Some(alt) = level.strip_suffix(" ft") {
        // Altitude
        alt.to_string() + "ALT"
    } else {
        // Flight level or SFC
        level.to_string()
    }
}

pub fn format_latlon(latlon: &str) -> String {
    format!("{}:{}:{} {} {}:{}:{} {}", &latlon[..2], &latlon[2..4], &latlon[4..6], &latlon[6..7], &latlon[7..10], &latlon[10..12], &latlon[12..14], &latlon[15..16])
}


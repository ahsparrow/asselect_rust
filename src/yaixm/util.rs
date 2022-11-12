use gloo_net::http::Request;
use gloo_net::Error;
use crate::yaixm::{IcaoType, LocalType, Yaixm};

pub async fn fetch_yaixm() -> Result<Yaixm, Error> {
    let result = Request::get("yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await,
        Err(e) => Err(e),
    }
}

pub fn rat_names(yaixm: &Yaixm) -> Vec<String> {
    let rat = &yaixm.rat;
    rat.iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn loa_names(yaixm: &Yaixm) -> Vec<String> {
    let loa = &yaixm.loa;
    loa.iter()
        .filter(|x| !x.default.unwrap_or(false))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn wav_names(yaixm: &Yaixm) -> Vec<String> {
    yaixm.airspace.iter()
        .filter(|x| x.icao_type == IcaoType::DOther &&
                x.local_type == Some(LocalType::Glider))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}

pub fn gliding_sites(yaixm: &Yaixm) -> Vec<String> {
    yaixm.airspace.iter()
        .filter(|x| x.icao_type == IcaoType::Other &&
                x.local_type == Some(LocalType::Glider))
        .map(|x| x.name.clone())
        .collect::<Vec<String>>()
}


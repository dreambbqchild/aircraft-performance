use lazy_static::lazy_static;

use serde::Deserialize;
use std::{collections::HashMap, fs::File};

lazy_static! {
    pub static ref AIRPORTS: HashMap<String, Airport> = load_airports();
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error> where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
} 

#[derive(Deserialize)]
pub struct Runway {
    pub airport_ref: u32,
    #[serde(rename = "length_ft", deserialize_with = "default_if_empty")]
    pub length: u16,
    pub surface: String,
    pub le_ident: String,
    #[serde(rename = "le_elevation_ft", deserialize_with = "default_if_empty")]
    pub le_elevation: i16,
    #[serde(rename = "le_heading_degT", deserialize_with = "default_if_empty")]
    le_heading_raw: f32,
    #[serde(skip)]
    pub le_heading: u16,
    #[serde(rename = "le_displaced_threshold_ft", deserialize_with = "default_if_empty")]
    pub le_displaced_threshold: u16,
    pub he_ident: String,
    #[serde(rename = "he_elevation_ft", deserialize_with = "default_if_empty")]
    pub he_elevation: i16,
    #[serde(rename = "he_heading_degT", deserialize_with = "default_if_empty")]
    he_heading_raw: f32,
    #[serde(skip)]
    pub he_heading: u16,
    #[serde(rename = "he_displaced_threshold_ft", deserialize_with = "default_if_empty")]
    pub he_displaced_threshold: u16,
    #[serde(skip)]
    pub is_grass: bool
}

#[derive(Deserialize)]
pub struct Airport {
    pub id: u32,
    pub ident: String,
    pub name: String,
    #[serde(rename = "latitude_deg")]
    pub latitude: f64,
    #[serde(rename = "longitude_deg")]
    pub longitude: f64,
    #[serde(rename = "elevation_ft", deserialize_with = "default_if_empty")]
    pub elevation: i16,
    #[serde(skip)]
    pub runways: Vec<Runway>
}

fn heading_from_runway_number(str: &String) -> u16 {
    match atoi::atoi::<u16>(str.as_bytes()) {
        Some(value) => value * 10,
        None => 0
    }
}

pub fn load_airports() -> HashMap<String, Airport> {
    let mut runways = HashMap::new();
    let mut runway_rdr = csv::Reader::from_reader(File::open("data/runways.csv").expect("To open the runways db"));
    for result in runway_rdr.deserialize() {
        let mut runway: Runway = result.expect("To deserialize a runway");

        if runway.le_ident != "" && runway.he_ident != "" {
            runway.le_heading = heading_from_runway_number(&runway.le_ident);
            runway.he_heading = heading_from_runway_number(&runway.he_ident);
        }

        if runway.le_heading == 0 {
            runway.le_heading = runway.le_heading_raw.round() as u16;
            runway.he_heading = runway.he_heading_raw.round() as u16;
        }

        runway.is_grass = runway.surface.contains("GRASS") || runway.surface.contains("TURF") || runway.surface.contains("SOD") || runway.surface.contains("GRS");

        if !runways.contains_key(&runway.airport_ref) {
            runways.insert(runway.airport_ref, Vec::<Runway>::new());
        }

        let vec = runways.get_mut(&runway.airport_ref).unwrap();
        vec.push(runway);
    }

    let mut airports = HashMap::new();
    let mut airport_rdr = csv::Reader::from_reader(File::open("data/airports.csv").expect("To open the airports db"));
    for result in airport_rdr.deserialize() {
        let mut airport: Airport = result.expect("To get the airport data from the row");

        if runways.contains_key(&airport.id) {
            let mut runways = runways.remove(&airport.id).unwrap(); 
            for i in 0..runways.len() {
                if runways[i].le_elevation == 0 {
                    runways[i].le_elevation = airport.elevation;
                }

                if runways[i].he_elevation == 0 {
                    runways[i].he_elevation = airport.elevation;
                }
            }

            airport.runways = runways;
        }

        airports.insert(airport.ident.clone(), airport);
    }

    airports
}

pub trait AirportHash {
    fn load_by_identifier<S: AsRef<str>>(&self, identifier: S) -> Option<&Airport>;
}

impl AirportHash for HashMap<String, Airport> {
    fn load_by_identifier<S: AsRef<str>>(&self, identifier: S) -> Option<&Airport> {
        let identifier_ref = identifier.as_ref();
        if self.contains_key(identifier_ref) {
            self.get(identifier_ref)
        }
        else {
            let short_identifier = &identifier_ref[1..];
            if self.contains_key(short_identifier) {
                self.get(short_identifier)
            } else {
                None
            }
        }
    }
}
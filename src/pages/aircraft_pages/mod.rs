use serde::Deserialize;

use crate::math::{Pressure, Temperature, Velocity};

pub mod cessna150j;
pub mod cessna172m;

#[derive(Deserialize)]
pub struct QueryPerformanceParameters {
    pub is_grass: Option<bool>,
    pub elevation_ft: i16,
    pub headwind_kts: i16,
    pub pressure_in_hg: Option<f32>,
    pub temperature_f: Option<i16>,
    pub temperature_c: Option<i16>,
    pub standard_temperature_f: Option<i16>,
    pub standard_temperature_c: Option<i16>,
    pub aircraft_weight_lbs: Option<i16>
}

impl QueryPerformanceParameters {
    fn convert_to_temperature_or_get_standard(c: Option<i16>, f: Option<i16>) -> Temperature {
        match c {
            Some(c) => Temperature::Celsius(c),
            None => match f {
                Some(f) => Temperature::Fahrenheit(f),
                None => Temperature::Fahrenheit(59)
            }
        }
    }

    pub fn to_performance_parameters(&self) -> PerformanceParameters {
        PerformanceParameters {
            is_grass: self.is_grass.unwrap_or(false),
            elevation_ft: self.elevation_ft,
            pressure: match self.pressure_in_hg { Some(in_hg) => Some(Pressure::InchesOfMercury(in_hg)), None => None },
            headwind: Velocity::Knots(self.headwind_kts),
            temperature: Self::convert_to_temperature_or_get_standard(self.temperature_c, self.temperature_f),
            standard_temperature: Self::convert_to_temperature_or_get_standard(self.standard_temperature_c, self.standard_temperature_f),
            aircraft_weight_lbs: self.aircraft_weight_lbs
        }
    }
}

pub struct PerformanceParameters {
    pub is_grass: bool,
    pub elevation_ft: i16,
    pub pressure: Option<Pressure>,
    pub headwind: Velocity,
    pub temperature: Temperature,
    pub standard_temperature: Temperature,
    pub aircraft_weight_lbs: Option<i16>
}

pub fn get_raw_html_for_take_off(aircraft_type: String, performance: PerformanceParameters, start_landing_flow: bool) -> String {
    match aircraft_type.as_str() {
        "cessna150j" => cessna150j::get_raw_html_for_take_off(&performance, start_landing_flow),
        "cessna172m" => cessna172m::get_raw_html_for_take_off(&performance, start_landing_flow),
        _ => "".to_string()
    }
}

pub fn get_raw_html_for_landing(aircraft_type: String, performance: PerformanceParameters) -> String {
    match aircraft_type.as_str() {
        "cessna150j" => cessna150j::get_raw_html_for_landing(&performance),
        "cessna172m" => cessna172m::get_raw_html_for_landing(&performance),
        _ => "".to_string()
    }
}
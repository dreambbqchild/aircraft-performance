use metar::{Wind, WindDirection, WindSpeed};

use super::Velocity;

fn get_heading_and_speed(wind: &Wind) -> (i16, i16) {
    let mut heading = 0;
    let mut speed = 0;
    if let WindDirection::Heading(h) = wind.dir.unwrap() {
        heading = *h as i16;
    }

    if let WindSpeed::Knot(v) = wind.speed.unwrap() {
        speed = *v as i16;
    }

    (heading, speed)
}

fn find_difference_in_radians(wind_heading: i16, heading: i16) -> f64 {
    let result = (wind_heading - heading) as f64;
    return result.to_radians()
}

pub trait WindCalcs {
    fn calc_crosswind_component(&self, heading: i16) -> i16;
    fn calc_headwind_component_from_metar_wind_value(&self, aircraft_heading: i16) -> Velocity;
}

impl WindCalcs for Wind {
    fn calc_crosswind_component(&self, heading: i16) -> i16 {
        let (wind_heading, wind_speed) = get_heading_and_speed(self);
        let rad = find_difference_in_radians(wind_heading, heading);
        (wind_speed as f64 * rad.sin()).round() as i16
    }

    fn calc_headwind_component_from_metar_wind_value(&self, aircraft_heading: i16) -> Velocity {
        let (wind_heading, wind_speed) = get_heading_and_speed(self);
        let rad = find_difference_in_radians(wind_heading, aircraft_heading);
        Velocity::Knots((wind_speed as f64 * rad.cos()).round() as i16)
    }
}
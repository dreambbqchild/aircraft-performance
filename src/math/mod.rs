use crate::data::performance::distance::Distance;

pub mod wind;

#[derive(Debug, Clone, Copy)]
pub enum Velocity {
    Knots(i16),
    MilesPerHour(i16)
}

impl Velocity {
    pub fn knots(self) -> Result<i16, &'static str> {
        if let Velocity::Knots(k) = self {
            Ok(k)
        } else if let Velocity::MilesPerHour(mph) = self {
            Ok((mph as f64 / 1.151).round() as i16)
        }
        else {
            Err("Can't convert")
        }
    }

    pub fn miles_per_hour(self) -> Result<i16, &'static str> {
        if let Velocity::Knots(k) = self {
            Ok((k as f64 * 1.151).round() as i16)
        } else if let Velocity::MilesPerHour(mph) = self {
            Ok(mph)
        }
        else {
            Err("Can't convert")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Fahrenheit(i16),
    Celsius(i16)
}

impl Temperature {
    pub fn celsius(self) -> Result<i16, &'static str> {
        if let Temperature::Celsius(c) = self {
            Ok(c)
        } else if let Temperature::Fahrenheit(f) = self {
            Ok(((f as f64 - 32.0) * 5.0 / 9.0).round() as i16)
        }
        else {
            Err("Can't convert")
        }
    }

    pub fn fahrenheit(self) -> Result<i16, &'static str> {
        if let Temperature::Celsius(c) = self {
            Ok(((c as f64 * 9.0 / 5.0) + 32.0).round() as i16)
        } else if let Temperature::Fahrenheit(f) = self {
            Ok(f)
        }
        else {
            Err("Can't convert")
        }
    }
}

pub trait FloatingCalcs {
    fn percent(&self, lower_bound: f64, upper_bound: f64) -> f64;
    fn percent_of(&self, lower_bound: f64, upper_bound: f64) -> f64;

    fn percent_i16(&self, lower_bound: i16, upper_bound: i16) -> f64;
    fn percent_of_i16(&self, lower_bound: i16, upper_bound: i16) -> f64;

    fn percent_of_distance(&self, lower_bound: Distance, upper_bound: Distance) -> Distance;
}

impl FloatingCalcs for f64 {
    fn percent(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        let diff = upper_bound - lower_bound;
        let actual_no_offset = self - lower_bound;
        match diff {
            0.0 => 0.0,
            _ => actual_no_offset as f64 / diff
        }
    }

    fn percent_of(&self, lower_bound: f64, upper_bound: f64) -> f64 {
        let diff = upper_bound - lower_bound;
        self * diff + lower_bound
    }

    fn percent_i16(&self, lower_bound: i16, upper_bound: i16) -> f64 {
        self.percent(lower_bound as f64, upper_bound as f64)
    }

    fn percent_of_i16(&self, lower_bound: i16, upper_bound: i16) -> f64 {
        self.percent_of(lower_bound as f64, upper_bound as f64)
    }

    fn percent_of_distance(&self, lower_bound: Distance, upper_bound: Distance) -> Distance {
        Distance::new_from_f64(
            self.percent_of_i16(lower_bound.ground_run(), upper_bound.ground_run()),
            self.percent_of_i16(lower_bound.clear_50_ft_obstacle(), upper_bound.clear_50_ft_obstacle())
        )
    }
}
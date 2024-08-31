use crate::data::performance::distance::Distance;

pub mod wind;

#[derive(Debug, Clone, Copy)]
pub enum Velocity {
    Knots(i16),
    MilesPerHour(i16)
}

impl Velocity {
    pub fn knots(self) -> i16 {
        match self {
            Velocity::Knots(k) => k,
            Velocity::MilesPerHour(mph) => (mph as f64 / 1.151).round() as i16,
        }
    }

    pub fn miles_per_hour(self) -> i16 {
        match self {
            Velocity::Knots(k) => (k as f64 * 1.151).round() as i16,
            Velocity::MilesPerHour(mph) => mph
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Temperature {
    Fahrenheit(i16),
    Celsius(i16)
}

impl Temperature {
    pub fn celsius(self) -> i16 {
        match self {
            Temperature::Celsius(c) => c,
            Temperature::Fahrenheit(f) => ((f as f64 - 32.0) * 5.0 / 9.0).round() as i16
        }
    }

    pub fn fahrenheit(self) -> i16 {
        match self {
            Temperature::Celsius(c) => ((c as f64 * 9.0 / 5.0) + 32.0).round() as i16,
            Temperature::Fahrenheit(f) => f
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pressure {
    Altimeter(f64),
    Altitude(f64)
}

impl Pressure {
    pub fn altimeter(self, elevation_ft: i16) -> f64 {
        match self {
            Pressure::Altimeter(p) => p,
            Pressure::Altitude(p) => (29.92 - p) * 1000.0 + elevation_ft as f64
        }   
    }

    pub fn altitude(self, elevation_ft: i16) -> f64 {
        match self {
            Pressure::Altimeter(p) => (p + elevation_ft as f64 + 29920.0) / 1000.0,
            Pressure::Altitude(p) => p
        }      
    }
}

pub trait FloatingCalcs {
    fn percent(&self, lower_bound: f64, upper_bound: f64) -> f64;
    fn percent_of(&self, lower_bound: f64, upper_bound: f64) -> f64;

    fn percent_i16(&self, lower_bound: i16, upper_bound: i16) -> f64;
    fn percent_of_i16(&self, lower_bound: i16, upper_bound: i16) -> f64;

    fn percent_velocity(&self, lower_bound: Velocity, upper_bound: Velocity) -> f64;

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

    fn percent_velocity(&self, lower_bound: Velocity, upper_bound: Velocity) -> f64 {
        self.percent_of(lower_bound.knots() as f64, upper_bound.knots() as f64)
    }

    fn percent_of_distance(&self, lower_bound: Distance, upper_bound: Distance) -> Distance {
        Distance::new_from_f64(
            self.percent_of_i16(lower_bound.ground_run(), upper_bound.ground_run()),
            self.percent_of_i16(lower_bound.clear_50_ft_obstacle(), upper_bound.clear_50_ft_obstacle())
        )
    }
}
use crate::{
    data::performance::{atmosphere_bounds::{AtmosphereBounds, AtmosphereDef}, 
    distance::Distance, headwinds::Headwinds, performance_row::PerformanceRow}, 
    math::{FloatingCalcs, Velocity}
};

fn find_headwind_performance_keys(headwind_kts: i16) -> Result<Headwinds, &'static str> {
    let lower_value = (headwind_kts / 10) * 10;
    let upper_value = lower_value + 10;    

    if lower_value < 0 || upper_value > 20  {
        Err("Unable to calculate, performance not defined")
    } else {

        Ok(Headwinds {
            lower_value: Velocity::Knots(lower_value),
            upper_value: Velocity::Knots(upper_value)
        }) 
    }
}

#[derive(Clone, Copy)]
pub enum Atmosphere {
    Alt0_59F = 0,
    Alt2500_50F = 1,
    Alt5000_41F = 2,
    Alt7500_32F = 3
}

const ALT0_59F: AtmosphereDef<Atmosphere> = AtmosphereDef { altitude: 0, temperature: 59, indexer: Atmosphere::Alt0_59F };
const ALT2500_50F: AtmosphereDef<Atmosphere> = AtmosphereDef { altitude: 2500, temperature: 50, indexer: Atmosphere::Alt2500_50F };
const ALT5000_41F: AtmosphereDef<Atmosphere> = AtmosphereDef { altitude: 5000, temperature: 41, indexer: Atmosphere::Alt5000_41F };
const ALT7500_32F: AtmosphereDef<Atmosphere> = AtmosphereDef { altitude: 7500, temperature: 32, indexer: Atmosphere::Alt7500_32F };

fn find_atmosphere(altitude_ft: i16) -> Result<AtmosphereBounds<Atmosphere>, &'static str> {
    if altitude_ft == 7500 {
        Ok(AtmosphereBounds { lower: ALT7500_32F, upper: ALT7500_32F })
    } else if altitude_ft > 5000 {
        Ok(AtmosphereBounds { lower: ALT5000_41F, upper: ALT7500_32F })
    } else if altitude_ft == 5000 {
        Ok(AtmosphereBounds { lower: ALT5000_41F, upper: ALT5000_41F })
    } else if altitude_ft > 2500 {
        Ok(AtmosphereBounds { lower: ALT2500_50F, upper: ALT5000_41F })
    } else if altitude_ft == 2500 {
        Ok(AtmosphereBounds { lower: ALT2500_50F, upper: ALT2500_50F })
    } else if altitude_ft > 0 {
        Ok(AtmosphereBounds { lower: ALT0_59F, upper: ALT2500_50F })
    } else if altitude_ft == 0 {
        Ok(AtmosphereBounds { lower: ALT0_59F, upper: ALT0_59F })
    } else {
        Err("Unable to calculate, performance not defined")
    }
}

fn get_take_off_distance(velocity: &Velocity, atmosphere: &Atmosphere) -> Result<Distance, &'static str> {
    match velocity {
        Velocity::Knots(0) => match atmosphere {
            Atmosphere::Alt0_59F => Ok(Distance(735, 1385)),
            Atmosphere::Alt2500_50F => Ok(Distance(910, 1660)),
            Atmosphere::Alt5000_41F => Ok(Distance(1115, 1985)),
            Atmosphere::Alt7500_32F => Ok(Distance(1360, 2440))
        },
        Velocity::Knots(10) => match atmosphere {
            Atmosphere::Alt0_59F => Ok(Distance(500, 1035)),
            Atmosphere::Alt2500_50F => Ok(Distance(630, 1250)),
            Atmosphere::Alt5000_41F => Ok(Distance(780, 1510)),
            Atmosphere::Alt7500_32F => Ok(Distance(970, 1875))
        },
        Velocity::Knots(20) => match atmosphere {
            Atmosphere::Alt0_59F => Ok(Distance(305, 730)),
            Atmosphere::Alt2500_50F => Ok(Distance(395, 890)),
            Atmosphere::Alt5000_41F => Ok(Distance(505, 1090)),
            Atmosphere::Alt7500_32F => Ok(Distance(640, 1375))
        },
        _ => Err("Undefined performance")
    }
}

fn get_landing_distance(atmosphere: &Atmosphere) -> Result<Distance, &'static str> {
    match atmosphere {
        Atmosphere::Alt0_59F => Ok(Distance(445, 1075)),
        Atmosphere::Alt2500_50F => Ok(Distance(470, 1135)),
        Atmosphere::Alt5000_41F => Ok(Distance(495, 1195)),
        Atmosphere::Alt7500_32F => Ok(Distance(520, 1255))
    }
}

pub struct Corrections {
    pub standard_temperature_correction_percentage: f64,
    pub distance_corrected_for_temperature: Distance,
    pub grass_offset: i16,
    pub distance_corrected_for_grass: Distance
}

pub struct TakeOff {
    pub takeoff_distances: [PerformanceRow; 3],
    pub distance_at_elevation: Distance,
    pub correction: Corrections
}

pub struct Landing {
    pub landing_distances: PerformanceRow,
    pub distance_at_elevation: Distance,
    pub headwind_correction_percentage: f64,
    pub distance_with_headwind: Distance,
    pub correction: Corrections
}

pub struct Cessna150J {
    pub headwind_kts: i16,
    pub headwinds: Headwinds,
    pub headwind_tween_percentage: f64,
    pub elevation_ft: i16,
    pub atmosphere_bounds: AtmosphereBounds<Atmosphere>,
    pub altitude_tween_percentage: f64,
    pub temperature_f: i16,
    pub standard_temperature_f: i16,
    pub temperature_f_diff_from_standard: i16
}

impl Cessna150J {
    pub fn new(headwind: Velocity, temperature_f: i16, elevation_ft: i16, standard_temperature_f: i16) -> Cessna150J {
        let headwind_kts = headwind.knots();
        let headwinds = find_headwind_performance_keys(headwind_kts).expect("To get the headwind bounds.");
        let wind_percentage = (headwind_kts as f64).percent_velocity(headwinds.lower_value, headwinds.upper_value);
        
        let atmosphere_bounds = find_atmosphere(elevation_ft).expect("To get the atmospheric bounds.");
        let altitude_tween_percentage = (elevation_ft as f64).percent_i16(atmosphere_bounds.lower.altitude, atmosphere_bounds.upper.altitude);

        Cessna150J {
            headwind_kts,
            headwinds,
            headwind_tween_percentage: wind_percentage,
            elevation_ft,
            atmosphere_bounds,
            altitude_tween_percentage,
            temperature_f,
            standard_temperature_f,
            temperature_f_diff_from_standard: temperature_f - standard_temperature_f
        }
    }
    
    fn calc_standard_temperature_correction_percentage(&self, standard_temperature_correction_interval: f64) -> f64 {
        (0.0f64).max(0.1 * (self.temperature_f_diff_from_standard as f64 / standard_temperature_correction_interval))
    }

    fn calc_distance_corrected_for_temperature(&self, distance: Distance, standard_temperature_correction_percentage: f64) -> Distance {
        Distance::new_from_f64(
            distance.ground_run() as f64 * (1.0 + standard_temperature_correction_percentage),
            distance.clear_50_ft_obstacle() as f64 * (1.0 + standard_temperature_correction_percentage))
    }

    fn calc_distance_corrected_for_grass(&self, distance: Distance, scale_factor: f64) -> (i16, Distance) {
        let grass_offset = (distance.clear_50_ft_obstacle() as f64 * scale_factor).round() as i16;
        (grass_offset, Distance(
            distance.ground_run() + grass_offset,
            distance.clear_50_ft_obstacle() + grass_offset,
        ))
    }

    pub fn calc_take_off(&self) -> TakeOff {
        let lower_row_lower_distance = get_take_off_distance(&self.headwinds.lower_value, &self.atmosphere_bounds.lower.indexer).expect("To get lower_wind_lower_altitude.");
        let lower_row_upper_distance = get_take_off_distance(&self.headwinds.lower_value, &self.atmosphere_bounds.upper.indexer).expect("To get lower_wind_upper_altitude.");
        let upper_row_lower_distance = get_take_off_distance(&self.headwinds.upper_value, &self.atmosphere_bounds.lower.indexer).expect("To get upper_wind_lower_altitude.");
        let upper_row_upper_distance = get_take_off_distance(&self.headwinds.upper_value, &self.atmosphere_bounds.upper.indexer).expect("To get upper_wind_upper_altitude.");

        let lower_row_middle_tween = self.altitude_tween_percentage.percent_of_distance(lower_row_lower_distance, lower_row_upper_distance);
        let upper_row_middle_tween = self.altitude_tween_percentage.percent_of_distance(upper_row_lower_distance, upper_row_upper_distance);

        let middle_row_lower_tween = self.headwind_tween_percentage.percent_of_distance(lower_row_lower_distance, upper_row_lower_distance);
        let middle_row_upper_tween = self.headwind_tween_percentage.percent_of_distance(lower_row_upper_distance, upper_row_upper_distance);
        let distance_at_elevation = self.altitude_tween_percentage.percent_of_distance(middle_row_lower_tween, middle_row_upper_tween);

        let takeoff_distances = [
            PerformanceRow::new_labeled(self.headwinds.lower_value.knots(), lower_row_lower_distance, lower_row_middle_tween, lower_row_upper_distance),
            PerformanceRow::new_labeled(self.headwind_kts, middle_row_lower_tween, distance_at_elevation, middle_row_upper_tween),
            PerformanceRow::new_labeled(self.headwinds.upper_value.knots(), upper_row_lower_distance, upper_row_middle_tween, upper_row_upper_distance)
        ];

        let standard_temperature_correction_interval = 35.0;
        let standard_temperature_correction_percentage = self.calc_standard_temperature_correction_percentage(standard_temperature_correction_interval);
        let distance_corrected_for_temperature = self.calc_distance_corrected_for_temperature(distance_at_elevation, standard_temperature_correction_percentage);        
        
        let distance_corrected_for_grass_scale_factor = 0.07;
        let (grass_offset, distance_corrected_for_grass) = self.calc_distance_corrected_for_grass(distance_corrected_for_temperature, distance_corrected_for_grass_scale_factor);

        TakeOff {
            takeoff_distances,
            distance_at_elevation,
            correction: Corrections {
                standard_temperature_correction_percentage,
                distance_corrected_for_temperature,
                grass_offset,
                distance_corrected_for_grass
            }
        }
    }

    pub fn calc_landing(&self) -> Landing {
        let lower_distance = get_landing_distance(&self.atmosphere_bounds.lower.indexer).expect("To get lower_altitude.");
        let upper_distance = get_landing_distance(&self.atmosphere_bounds.upper.indexer).expect("To get upper_altitude.");
        
        let distance_at_elevation = self.altitude_tween_percentage.percent_of_distance(lower_distance, upper_distance);

        let landing_distances = PerformanceRow::new_unlabeled(lower_distance, distance_at_elevation, upper_distance);

        let headwind_correction_percentage = (self.headwind_kts as f64 / 4.0) * 0.1;
        let distance_with_headwind = Distance::new_from_f64(
        distance_at_elevation.ground_run() as f64 * (1.0 - headwind_correction_percentage), 
        distance_at_elevation.clear_50_ft_obstacle() as f64 * (1.0 - headwind_correction_percentage));

        let standard_temperature_correction_interval = 60.0;
        let standard_temperature_correction_percentage = self.calc_standard_temperature_correction_percentage(standard_temperature_correction_interval);
        let distance_corrected_for_temperature = self.calc_distance_corrected_for_temperature(distance_with_headwind, standard_temperature_correction_percentage);

        let distance_corrected_for_grass_scale_factor = 0.2;
        let (grass_offset, distance_corrected_for_grass) = self.calc_distance_corrected_for_grass(distance_corrected_for_temperature, distance_corrected_for_grass_scale_factor);

        Landing {
            landing_distances,
            headwind_correction_percentage,
            distance_with_headwind,
            distance_at_elevation: distance_at_elevation,
            correction: Corrections {
                standard_temperature_correction_percentage,
                distance_corrected_for_temperature,
                grass_offset,
                distance_corrected_for_grass
            }
        }
    }
}
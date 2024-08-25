use super::distance::Distance;

pub struct PerformanceRow {
    pub wind_speed: Option<i16>,
    pub lower_bound: Distance,
    pub middle_value: Option<Distance>,
    pub upper_bound: Distance
}

impl PerformanceRow {
    fn new(wind_speed: Option<i16>, lower_bound: Distance, middle_value: Option<Distance>, upper_bound: Distance) -> Self {
        PerformanceRow {
            wind_speed,
            lower_bound,
            middle_value,
            upper_bound
        }
    }

    pub fn new_labeled(wind_speed: i16, lower_bound: Distance, middle_value: Option<Distance>, upper_bound: Distance) -> Self {
        Self::new(Some(wind_speed), lower_bound, middle_value, upper_bound)
    }

    pub fn new_unlabeled(lower_bound: Distance, middle_value: Option<Distance>, upper_bound: Distance) -> Self {
        Self::new(None, lower_bound, middle_value, upper_bound)
    }
}
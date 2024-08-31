use super::distance::Distance;

pub struct PerformanceRow {
    pub label: Option<i16>,
    pub lower_bound: Distance,
    pub middle_value: Distance,
    pub upper_bound: Distance
}

impl PerformanceRow {
    fn new(label: Option<i16>, lower_bound: Distance, middle_value: Distance, upper_bound: Distance) -> Self {
        PerformanceRow {
            label,
            lower_bound,
            middle_value,
            upper_bound
        }
    }

    pub fn new_labeled(label: i16, lower_bound: Distance, middle_value: Distance, upper_bound: Distance) -> Self {
        Self::new(Some(label), lower_bound, middle_value, upper_bound)
    }

    pub fn new_unlabeled(lower_bound: Distance, middle_value: Distance, upper_bound: Distance) -> Self {
        Self::new(None, lower_bound, middle_value, upper_bound)
    }
}
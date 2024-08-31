#[derive(Clone, Copy)]
pub struct Distance(pub i16, pub i16);

impl Distance {
    pub fn new_from_f64(ground_run: f64, clear_50_ft_obstacle: f64) -> Distance {
        Distance(ground_run.round() as i16, clear_50_ft_obstacle.round() as i16)
    }
    
    pub fn ground_run(&self) -> i16 {
        self.0
    }

    pub fn clear_50_ft_obstacle(&self) -> i16 {
        self.1
    }
}
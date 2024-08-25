#[derive(Clone, Copy)]
pub struct Distance {
    ground_run: i16,
    clear_50_ft_obstacle: i16
}

impl Distance {
    pub fn new(ground_run: i16, clear_50_ft_obstacle: i16) -> Distance {
        Distance {
            ground_run: ground_run, 
            clear_50_ft_obstacle: clear_50_ft_obstacle 
        }
    }

    pub fn new_from_f64(ground_run: f64, clear_50_ft_obstacle: f64) -> Distance {
        Distance {
            ground_run: ground_run.round() as i16, 
            clear_50_ft_obstacle: clear_50_ft_obstacle.round() as i16 
        }
    }
    
    pub fn ground_run(self) -> i16 {
        self.ground_run
    }

    pub fn clear_50_ft_obstacle(self) -> i16 {
        self.clear_50_ft_obstacle
    }
}
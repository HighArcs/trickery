pub struct TemperatureMonth(Vec<Vec<f64>>);

impl TemperatureMonth {
    pub fn new(values: Vec<Vec<f64>>) -> Self {
        Self(values)
    }

    pub fn get_max_temp_weekly(&self) -> Vec<f64> {
        let mut weeks = Vec::new();
        for i in &self.0 {
            let mut max = f64::NEG_INFINITY;
            for j in i {
                if j > &max {
                    max = *j;
                }
            }

            weeks.push(max);
        }

        weeks
    }

    pub fn get_min_temp_weekly(&self) -> Vec<f64> {
        let mut weeks = Vec::new();
        for i in &self.0 {
            let mut min = f64::INFINITY;
            for j in i {
                if j < &min {
                    min = *j;
                }
            }

            weeks.push(min);
        }

        weeks
    }
    
    pub fn get_range(&self) -> f64 {
        let mut max = f64::NEG_INFINITY;
        let mut min = f64::INFINITY;

        for i in &self.0 {
            for j in i {
                if j > &max {
                    max = *j;
                }

                if j < &min {
                    min = *j;
                }
            }
        }

        max - min
    }
    
    pub fn get_certain_temp(&self, week: usize, day: usize) -> &f64 {
        &self.0[week][day]
    }
}

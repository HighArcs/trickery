use std::{f64, fmt::Display};

pub struct Airplane {
    distance: f64,
    direction: i32,

    altitude: i32,
    call_sign: String,
}

fn floor_mod(x: i32, y: i32) -> i32 {
    x - ((((x as f64) / (y as f64)).floor() as i32) * y)
}

impl Airplane {
    pub fn new(call_sign: String, distance: f64, direction: i32, altitude: i32) -> Self {
        Self {
            call_sign,
            distance: distance.abs(),
            altitude: altitude.abs(),
            direction: direction % 360,
        }
    }

    pub fn gain_alt(&mut self) {
        self.altitude += 1000;
    }

    pub fn lose_alt(&mut self) {
        self.altitude -= self.altitude.min(1000)
    }

    pub fn get_alt(&self) -> i32 {
        self.altitude
    }

    pub fn mov(&mut self, dist: f64, dir: i32) {
        let r1 = self.distance;
        let r2 = dist;

        let u1 = (self.direction as f64).to_radians();
        let u2 = (dir as f64).to_radians();

        let r1s = r1.powi(2);
        let r2s = r2.powi(2);

        let r2c = 2.0 * r1 * r2 * (u2 - u1).cos();

        self.distance = (r1s + r2s + r2c).sqrt();

        let y1 = r1 * u1.sin();
        let y2 = r2 * u2.sin();
        let x1 = r1 * u1.cos();
        let x2 = r2 * u2.cos();

        let ang = (y1 + y2).atan2(x1 + x2);

        self.direction = floor_mod(ang.to_degrees().round() as i32, 360)
    }

    pub fn dist_to(&self, other: &Self) -> f64 {
        let r1 = self.distance;
        let r2 = other.distance;

        let u1 = (self.direction as f64).to_radians();
        let u2 = (other.direction as f64).to_radians();

        let r1s = r1.powi(2);
        let r2s = r2.powi(2);
        let r2c = 2.0 * r1 * r2 * (u2 - u1).cos();

        let between = (r1s + r2s - r2c).sqrt();

        (100.0 * between).round() / 100.0
    }
}

impl Default for Airplane {
    fn default() -> Self {
        Self::new("AAA01".to_string(), 1.0, 0, 0)
    }
}

impl Display for Airplane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {:.2} miles away at bearing {:03}\u{00b0}, altitude {} feet",
            self.call_sign, self.distance, self.direction, self.altitude
        )
    }
}

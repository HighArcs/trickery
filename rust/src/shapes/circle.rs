use std::fmt::Display;

pub struct Circle(f64);

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self(radius)
    }

    pub fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.get_radius().powi(2)
    }

    pub fn get_circumference(&self) -> f64 {
        std::f64::consts::TAU * self.get_radius()
    }

    pub fn get_radius(&self) -> f64 {
        self.0
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.0 = radius
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Display for Circle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "circle with radius {}", self.get_radius())
  }
}
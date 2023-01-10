use std::fmt::Display;

pub struct Rectangle(f64, f64);

impl Rectangle {
    pub fn new(length: f64, width: f64) -> Self {
        Self(length, width)
    }

    pub fn square(size: f64) -> Self {
        Self(size, size)
    }

    pub fn get_area(&self) -> f64 {
        self.get_length() * self.get_width()
    }

    pub fn get_perimeter(&self) -> f64 {
        (2.0 * self.get_length()) + (2.0 * self.get_width())
    }

    pub fn get_length(&self) -> f64 {
        self.0
    }

    pub fn get_width(&self) -> f64 {
        self.1
    }

    pub fn set_length(&mut self, length: f64) {
        self.0 = length
    }

    pub fn set_width(&mut self, width: f64) {
        self.1 = width
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self(1.0, 1.0)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rectangle with length {}, width {}",
            self.get_length(),
            self.get_width()
        )
    }
}

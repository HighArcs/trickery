use std::fmt::Display;

pub struct RegularPolygon {
    sides: u32,
    length: f64,
}

impl RegularPolygon {
    pub fn new(sides: u32, length: f64) -> Self {
        Self { sides, length }
    }

    pub fn of_sides(sides: u32) -> Self {
        Self { sides, length: 1.0 }
    }

    pub fn of_length(length: f64) -> Self {
        Self { sides: 3, length }
    }

    pub fn add_side(&mut self) {
        self.add_sides(1)
    }

    pub fn add_sides(&mut self, amount: u32) {
        self.sides += amount;
    }

    pub fn get_area(&self) -> f64 {
        let apothem = self.length / (2.0 * (std::f64::consts::PI / (self.sides as f64)).tan());
        (apothem * self.get_perimeter()) / 2.0
    }

    pub fn get_perimeter(&self) -> f64 {
        (self.sides as f64) * self.length
    }

    pub fn get_side_length(&self) -> f64 {
        self.length
    }

    pub fn get_num_sides(&self) -> u32 {
        self.sides
    }

    pub fn set_num_sides(&mut self, sides: u32) {
        self.sides = sides;
    }

    pub fn set_side_length(&mut self, length: f64) {
        self.length = length;
    }
}

impl PartialEq for RegularPolygon {
    fn eq(&self, other: &Self) -> bool {
        self.get_side_length() == other.get_side_length()
            && self.get_num_sides() == other.get_num_sides()
    }
}

impl Display for RegularPolygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.sides {
            3 => write!(f, "triangle"),
            4 => write!(f, "square"),
            5 => write!(f, "pentagon"),
            6 => write!(f, "hexagon"),
            7 => write!(f, "septagon"),
            8 => write!(f, "octagon"),
            9 => write!(f, "nonagon"),
            10 => write!(f, "decagon"),
            c => write!(f, "{c} sided polygon"),
        }
    }
}

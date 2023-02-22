use std::fmt::Display;

use crate::tools::Scanner;

pub struct Plane {
    location: i32,
}

impl Plane {
    pub fn new(loc: i32) -> Self {
        if loc >= -2000 && loc <= 2000 {
            Self { location: loc }
        } else {
            Self { location: 0 }
        }
    }

    pub fn upward(&mut self) {
        self.location += 100;

        if self.location > 2000 {
            self.location = 2000;
        }
    }

    pub fn downward(&mut self) {
        self.location -= 100;

        if self.location < -2000 {
            self.location = -2000;
        }
    }

    pub fn get_location(&mut self) -> i32 {
        self.location
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in (-2000..self.location).step_by(100) {
            write!(f, " ")?;
        }

        write!(f, "@")
    }
}

pub fn activity() {
    let scan = Scanner::new();

    let mut p = Plane::default();
    let mut instruction = String::new();

    while instruction != "q" {
        writeln!(f, "{p}");
        writeln!(f, "Location: {}", p.get_location());
        writeln!(f, "Type \"u\" to move upwards, \"d\" to move downwards, \"n\" for new Plane, \"q\" to quit.");

        instruction = scan.next_line();

        match &*instruction {
            "u" => {p.upward();},
            "d" => {p.downward();},
            "n" => {
                writeln!(f, "Starting location for a new plane?");
                let start = scan.next_i32();
                p = Plane::new(start);
                scan.next_line();
            },
            "q" => continue,
            _ => writeln!(f, "Instruction not recognized")
        }
    }
}

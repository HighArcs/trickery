use std::fmt::Display;

use crate::tools::Scanner;

pub struct Rectangle {
    bs: f64,
    ht: f64,
}

impl Rectangle {
    pub fn new(mut bs: f64, mut ht: f64) -> Self {
        if bs < 0.0 {
            bs = 0.0;
        }

        if ht < 0.0 {
            ht = 0.0;
        }

        Self { bs, ht }
    }

    pub fn get_area(&self) -> f64 {
        self.get_base() * self.get_height()
    }

    pub fn get_base(&self) -> f64 {
        self.bs
    }

    pub fn get_height(&self) -> f64 {
        self.ht
    }

    pub fn get_diagonal(&self) -> f64 {
        (self.bs.powi(2) + self.ht.powi(2)).sqrt()
    }

    pub fn get_perimeter(&self) -> f64 {
        2.0 * self.get_base() + 2.0 * self.get_height()
    }

    pub fn set_base(&mut self, bs: f64) {
        if bs > 0.0 {
            self.bs = bs;
        }
    }

    pub fn set_height(&mut self, ht: f64) {
        if ht > 0.0 {
            self.ht = ht
        }
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        other.get_base() == self.get_base() && other.get_height() == self.get_height()
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "rectangle with base {}, height {}",
            self.get_base(),
            self.get_height()
        )
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { bs: 1.0, ht: 1.0 }
    }
}

pub fn activity() {
    let scan = Scanner::new();
    let mut r = Rectangle::default();
    let mut instruction = String::new();

    while instruction != "q" {
        writeln!(f, 
            "Type the name of the method to test. Type c to construct a new rectangle, q to quit."
        );

        instruction = scan.next_line();

        match &*instruction {
            "get_area" => {
                writeln!(f, "{}", r.get_area());
            }

            "get_base" => {
                writeln!(f, "{}", r.get_base());
            }

            "get_height" => {
                writeln!(f, "{}", r.get_height());
            }

            "get_perimeter" => {
                writeln!(f, "{}", r.get_perimeter());
            }

            "to_string" => {
                writeln!(f, "{}", r.to_string());
            }

            "get_diagonal" => {
                writeln!(f, "{}", r.get_diagonal())
            }

            "set_base" => {
                writeln!(f, "Enter parameter value:");
                let arg = scan.next_double();
                r.set_base(arg);
                scan.next_line();
            }

            "set_height" => {
                writeln!(f, "Enter parameter value:");
                let arg = scan.next_double();
                r.set_height(arg);
                scan.next_line();
            }

            "eq" => {
                writeln!(f, "Enter base and height:");
                let bs = scan.next_double();
                let ht = scan.next_double();

                let r_other = Rectangle::new(bs, ht);

                writeln!(f, "{}", r == r_other);
                scan.next_line();
            }

            "c" => {
                writeln!(f, "Enter base and height:");
                let bs = scan.next_double();
                let ht = scan.next_double();

                r = Rectangle::new(bs, ht);
                scan.next_line();
            }

            "q" => {
                break;
            }
            _ => writeln!(f, "Not recognized."),
        }
    }
}

use std::{
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::tools::Scanner;

fn next_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(1);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub struct Car {
    make: String,
    model: String,
    year: u16,
    mpg: f64,
    id: u32,
}

impl Car {
    pub fn new(make: String, model: String, mut year: u16, mut mpg: f64) -> Self {
        let id = next_id();
        if year > 2022 {
            year = 2022;
        }

        if year < 1885 {
            year = 2000;
        }

        if mpg < 0.0 {
            mpg = 0.0;
        }

        Self {
            make,
            model,
            year,
            mpg,
            id,
        }
    }
}

impl Default for Car {
    fn default() -> Self {
        Self::new("None".to_string(), "None".to_string(), 0, 0.0)
    }
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nMake: {}\nModel: {}\nYear: {}\nMPG: {}",
            self.id, self.make, self.model, self.year, self.mpg
        )
    }
}

pub fn activity() {
    let scan = Scanner::new();
    let mut i = String::new();

    while i != "q" {
        writeln!(f, "Input the make of the car, \"default\" to create a default car or \"q\" to quit:");
        i = scan.next_line();

        writeln!(f, );
        match &*i {
            "q" => {
                writeln!(f, "Exiting. Bye!");
            }

            "default" => {
                let car = Car::default();
                write!(f, "{}", car);
            }

            _ => {
                writeln!(f, "Model of the car:");
                let model = scan.next_line();

                writeln!(f, "Year of the car:");
                let year = scan.next_u16();

                writeln!(f, "Miles per gallon of the car:");
                let mpg = scan.next_double();

                writeln!(f, );
                writeln!(f, "{}", Car::new(i.to_owned(), model, year, mpg))
            }
        }

        writeln!(f, );
    }
}
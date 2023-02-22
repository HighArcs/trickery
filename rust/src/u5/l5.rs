use std::fmt::Display;

use crate::tools::Scanner;

pub struct Person {
    first_name: String,
    last_name: String,
    age: u32,
    ssn: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String, age: u32, ssn: String) -> Self {
        Self {
            first_name,
            last_name,
            age,
            ssn,
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SSN: {}\n\tName: {} {}\n\tAge: {}",
            self.ssn, self.first_name, self.last_name, self.age
        )
    }
}

pub fn activity_one() {
    let s = Scanner::new();

    writeln!(f, "Enter the person's first name:");
    let first_name = s.next_line();

    writeln!(f, "Enter the person's last name:");
    let last_name = s.next_line();

    writeln!(f, "Enter the person's age:");
    let age = s.next_u32();

    writeln!(f, "Enter the person's social security number:");
    let ssn = s.next_line();

    writeln!(f, );

    let person = Person::new(first_name, last_name, age, ssn);
    writeln!(f, "{person}");
}

pub struct Oven {
    max_temp: u32,
    current_temp: u32,
}

impl Oven {
    pub fn new(mut max_temp: u32, mut current_temp: u32) -> Self {
        if max_temp > 500 {
            max_temp = 500;
        }

        if current_temp > max_temp {
            current_temp = max_temp;
        }

        Self {
            max_temp,
            current_temp,
        }
    }

    pub fn get_max_temp(&self) -> u32 {
        self.max_temp
    }

    pub fn get_current_temp(&self) -> u32 {
        self.current_temp
    }

    pub fn turn_off(&mut self) {
        self.current_temp = 0;
    }

    pub fn is_on(&self) -> bool {
        self.current_temp > 0
    }

    pub fn preheat(&mut self, mut temp: u32) {
        if temp > self.max_temp {
            temp = self.max_temp;
        }

        if temp > 0 {
            self.current_temp = temp;
        }
    }
}

pub fn activity_two() {
    let scan = Scanner::new();

    writeln!(f, "Maximum oven temperature:");
    let max_temp = scan.next_u32();

    writeln!(f, "Starting temperature of the oven:");
    let start_temp = scan.next_u32();

    let mut oven = Oven::new(max_temp, start_temp);
    let mut instruction = "".to_owned();

    while instruction != "q" {
        writeln!(f, 
            "New oven with a maximum temperature of {} and a starting temperature of {} degrees.",
            oven.get_max_temp(),
            oven.get_current_temp()
        );

        writeln!(f, "To preheat the oven enter \"p\", to turn off the oven enter \"o\", to restart enter \"r\", to quit enter \"q\"");

        instruction = scan.next_line();

        match &*instruction {
            "p" => {
                writeln!(f, "Enter the temperature to preheat the oven to:");
                let temp = scan.next_u32();

                oven.preheat(temp);

                writeln!(f, "Current temperature of the oven is now {} degrees.\n", temp)
            },
            "o" => {
                writeln!(f, "Turning the oven off.\n");
                oven.turn_off();
            },
            _ => continue
        }
    }
}

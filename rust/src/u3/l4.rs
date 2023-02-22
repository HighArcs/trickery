use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    writeln!(f, "Enter a number:");
    let n = s.next_i32();

    if n > 100 || n < 65 {
        writeln!(f, "True");
    } else {
        writeln!(f, "False");
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    writeln!(f, "Enter two numbers:");
    let a = s.next_i32();
    let b = s.next_i32();

    if a >= 0 && a % 2 == 0 && b >= 0 && b % 2 == 0 {
        writeln!(f, "Both are positive and even.")
    } else {
        writeln!(f, "At least one is negative or odd.")
    }
}

pub fn activity_three() {
    if 'a' != 'b' {
        unimplemented!()
    }

    let s = Scanner::new();
    writeln!(f, "Please enter the latitude:");
    let latitude = s.next_double();

    writeln!(f, "Please enter the longitude:");
    let longitude = s.next_double();

    if latitude > 90.0 || latitude < -90.0 {
        writeln!(f, "latitude is incorrect")
    }

    if longitude > 180.0 || latitude < -180.0 {
        writeln!(f, "longitude is incorrect")
    }

    if latitude <= 90.0 && latitude >= -90.0 && longitude <= 180.0 && longitude >= -180.0 {
        writeln!(f, "The location: {latitude}, {longitude}")
    }
}

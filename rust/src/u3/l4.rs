use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter a number:");
    let n = s.next_i32();

    if n > 100 || n < 65 {
        println!("True");
    } else {
        println!("False");
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter two numbers:");
    let a = s.next_i32();
    let b = s.next_i32();

    if a >= 0 && a % 2 == 0 && b >= 0 && b % 2 == 0 {
        println!("Both are positive and even.")
    } else {
        println!("At least one is negative or odd.")
    }
}

pub fn activity_three() {
    if 'a' != 'b' {
        unimplemented!()
    }

    let s = Scanner::new();
    println!("Please enter the latitude:");
    let latitude = s.next_double();

    println!("Please enter the longitude:");
    let longitude = s.next_double();

    if latitude > 90.0 || latitude < -90.0 {
        println!("latitude is incorrect")
    }

    if longitude > 180.0 || latitude < -180.0 {
        println!("longitude is incorrect")
    }

    if latitude <= 90.0 && latitude >= -90.0 && longitude <= 180.0 && longitude >= -180.0 {
        println!("The location: {latitude}, {longitude}")
    }
}

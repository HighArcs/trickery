use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter a positive integer");
    let min = 2;
    let max = s.next_i32() + 2;

    println!(
        "{}",
        (rand::random::<f64>() * ((max - min + 1) as f64) + (min as f64)) as i32
    );
    println!(
        "{}",
        (rand::random::<f64>() * ((max - min + 1) as f64) + (min as f64)) as i32
    );
    println!(
        "{}",
        (rand::random::<f64>() * ((max - min + 1) as f64) + (min as f64)) as i32
    );
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter the first x-coordinate:");
    let x1 = s.next_double();

    println!("Enter the second x-coordinate:");
    let x2 = s.next_double();

    println!("Enter the first y-coordinate:");
    let y1 = s.next_double();

    println!("Enter the second y-coordinate:");
    let y2 = s.next_double();

    let dy = y2 - y1;
    let dx = x2 - x1;
    let slope = dy / dx;

    println!("Slope: {slope}")
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Enter two doubles:");
    let x = s.next_double();
    let y = s.next_double();

    let d = (x - y).abs().round() as i32;

    println!("Difference: {d}");
}

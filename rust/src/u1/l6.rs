use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    let n = s.next_i32();
    let d = s.next_i32();

    println!("The decimal value is: {}", (n as f64) / (d as f64));
}

pub fn activity_two() {
    let s = Scanner::new();

    let a = s.next_double().round() as i32;
    let b = s.next_double().round() as i32;

    println!("Answer: {} + {} = {}", a, b, a + b);
}

pub fn activity_three() {
    let s = Scanner::new();
    let d = s.next_double();

    let a = (d * 10.0 % 10.0) as i32;
    let b = (d * 100.0 % 10.0) as i32;
    let c = (d * 1000.0 % 10.0) as i32;

    println!("Answer: {} {} {}", a, b, c);
}

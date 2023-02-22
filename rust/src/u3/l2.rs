use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    writeln!(f, "Please enter five numbers:");

    let a = s.next_double();
    let b = s.next_double();
    let c = s.next_double();
    let d = s.next_double();
    let e = s.next_double();

    let average = (a + b + c + d + e) / 5.0;

    if average >= 89.95 {
        writeln!(f, "ABOVE AVERAGE")
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    writeln!(f, "Please enter two numbers:");

    let a = s.next_double();
    let b = s.next_double();

    if a <= b {
        writeln!(f, "Smallest is: {a}");
    }

    if b < a {
        writeln!(f, "Smallest is: {b}");
    }
}

pub fn activity_three() {
    let s = Scanner::new();

    writeln!(f, "What is the temperature?");
    let temp = s.next_double();

    if temp < 97.0 {
        writeln!(f, "NOT NORMAL");
    }

    if temp > 99.0 {
        writeln!(f, "NOT NORMAL")
    }
}

pub fn activity_four() {
    let s = Scanner::new();

    writeln!(f, "Enter two test scores:");

    let a = s.next_double();
    let b = s.next_double();

    if a < 0.0 {
        writeln!(f, "Not Valid");
    }

    if b < 0.0 {
        writeln!(f, "Not Valid");
    }

    if a > 100.0 {
        writeln!(f, "Not Valid");
    }

    if b > 100.0 {
        writeln!(f, "Not Valid");
    }
}

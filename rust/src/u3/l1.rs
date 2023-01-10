use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Please enter a double:");
    let d = s.next_double();

    if d == 12.345 {
        println!("YES");
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Please enter an integer:");
    let n = s.next_i32();

    if n == 48 {
        println!("YES");
    }
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Please enter two integers:");

    let a = s.next_i32();
    let b = s.next_i32();

    if b == (a * 2) {
        println!("YES");
    }
}

pub fn activity_four() {
    let s = Scanner::new();

    println!("Please enter an integer:");
    let n = s.next_i32();

    if n % 2 == 0 {
        println!("Divisible by 2!");
    }

    if n % 3 == 0 {
        println!("Divisible by 3!");
    }
}

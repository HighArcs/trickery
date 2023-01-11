use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter a number in the fifties");

    let mut n = s.next_i32();

    if n < 50 || n >= 60 {
        println!("That's not in the fifties!");
        n = 55;
    }

    println!("Your number is {n}");
}

pub fn activity_two() {
    let s = Scanner::new();

    let x = s.next_i32();
    let y = s.next_i32();

    if y <= 9 || (x > 2 && x * y > 10) {
        println!("pass")
    }
}

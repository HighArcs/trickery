use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("What is your favourite food?");

    let content = s.next_line();

    println!("I like to eat {} as well!", content);
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Please enter four names:");

    let a = s.next_line();
    let b = s.next_line();
    let c = s.next_line();
    let d = s.next_line();

    println!("{} {} {} {}", d, b, c, a);
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Hi there. What is your name?");
    let name = s.next_line();

    println!("What state do you live in?");
    let location = s.next_line();

    println!("My name is {}. I live in {}.", name, location);
}

use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Print 3 doubles:");

    let a = s.next_double();
    let b = s.next_double();
    let c = s.next_double();

    println!("{} {} {}", c, b, a)
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Hi there. What is your name?");

    let name = s.next_line();

    println!("Hi {}. How old are you?", name);
    let age = s.next_u8();

    println!("{} is {} years old.", name, age)
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Java is an object-oriented programming language, true or false?");
    let u = s.next_bool();

    println!(
        "There are only 2 possible values which can be held by a boolean variable, true or false?"
    );
    let p = s.next_bool();

    println!("Question 1 - Your answer: {}. Correct answer: true", u);
    println!("Question 2 - Your answer: {}. Correct answer: true", p);
}

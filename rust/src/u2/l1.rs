use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("What is your name?");
    let name = s.next_line();

    println!("What is your favourite number?");
    let n = s.next_i32();

    println!("Your name is {} and you like the number {}.", name, n)
}

pub fn activity_two() {
    let s = Scanner::new();

    let mut order = String::from("apple pie");

    println!("The current order is {}", order);

    println!("I want to eat something else, what do you want to eat?");
    order = s.next_line();

    println!("The otder has changed to {}", order)
}

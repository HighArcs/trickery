use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter a string:");
    let content = s.next_line();

    println!("Enter a number:");
    let location = s.next_usize();

    println!("{}{}", &content[0..location], &content[location..])
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter a string:");
    let content = s.next_line();

    println!("How many characters would you like to delete at the end?");
    let count = s.next_usize();

    println!("{}", &content[0..count]);
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Enter first word:");
    let a = s.next_line().to_lowercase();

    println!("Enter second word:");
    let b = s.next_line().to_lowercase();

    println!("Result: {:?}", a.cmp(&b))
}

pub fn activity_four() {
    let s = Scanner::new();

    println!("Enter a sentence:");
    let content = s.next_line();

    let word = content.split_ascii_whitespace().next().unwrap();

    println!("{}", word.len())
}

use crate::tools::Scanner;

pub fn activity_one() {
    println!("Enter any numbers (Enter 5 to stop)");
    let s = Scanner::new();

    let mut sum = 0;

    loop {
        let n = s.next_i32();

        if n == 5 {
            break;
        }

        sum += n;
    }

    println!("Sum is {sum}");
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter the Scores:");
    let mut largest = i32::MIN;

    loop {
        let n = s.next_i32();

        if n == -1 {
            break;
        }

        if n > largest {
            largest = n;
        }
    }

    println!("The largest score is {largest}")
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Input a word:");
    let word = s.next_line();

    let mut i = 0;
    while i < word.len() {
        if i == 0 || i % 3 != 2 {
            print!("{}", &word[i..i + 1])
        }

        i += 1;
    }
}

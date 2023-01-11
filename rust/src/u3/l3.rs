use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Please enter an integer");
    let n = s.next_i32();

    if n % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Please enter a letter grade:");
    let letter = s.next_line();

    println!(
        "{}",
        match &*letter {
            "A" => "90-100",
            "B" => "80-90",
            "C" => "70-80",
            "D" => "60-70",
            "F" => "0-60",
            _ => "Invalid letter grade",
        }
    )
}

pub fn activity_three() {}
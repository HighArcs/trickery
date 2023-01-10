use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("What type of item are you buying?");
    let item = s.next_line();

    println!("How many are you buying?");
    let count = s.next_i32();

    println!("How much does each one weight");
    let weight = s.next_double();

    println!(
        "{} {} at {} pounds each will weigh {} pounds total",
        count,
        item,
        weight,
        (count as f64) * weight
    )
}

pub fn activity_two() {
    println!(
        "{}{}{}",
        "\"That brain of mine is something more than merely mortal; ",
        "as time will show.\"\nAda Lovelace",
        "\nThe first computer programmer"
    )
}

pub fn activity_three() {
  print!("(\\(\\\n( - -)\n((') (')");
}
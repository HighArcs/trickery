use crate::tools::Scanner;

pub fn activity_one() {
    for i in (1..=25).step_by(2) {
        println!("{i}")
    }
}

pub fn activity_two() {
    for i in 17..=73 {
        if i % 10 == 6 {
            println!("{i}");
        } else {
            print!("{i} ")
        }
    }
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Enter a number between 0 and 50:");
    let n = s.next_i32();

    if n <= 0 || n >= 50 {
        println!("error");
    } else {
        for i in n..=50 {
            if i % 5 == n % 5 {
                print!("\n");
            }

            print!("{i} ")
        }
    }
}

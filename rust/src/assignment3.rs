use crate::tools::Scanner;

pub fn assignment3() {
    let s = Scanner::new();
    println!("Welcome. what is your name?");
    let name = s.next_line();

    println!("Hello {name}. Are you ready to crack the code?");
    let is_ready = s.next_line().to_lowercase() == "yes";

    if !is_ready {
        return;
    }

    println!();
    println!("PHASE 1");
    println!("Enter a string: ");
    let content = s.next_line();

    if content.len() == 3 {
        println!("Correct!");

        println!();
        println!("PHASE 2");
        println!("Enter a number:");

        let n = s.next_i32();
        if n == 19 || (n >= 35 && n < 78) {
            println!("Correct!");

            println!();
            println!("PHASE 3");
            println!("Enter a number:");

            let u = s.next_i32();

            if u > 0 && (u % 2 == 0 || u % 10 == 1) {
                println!("Correct!");
                println!("You have cracked the code!");
            } else {
                incorrect();
            }
        } else {
            incorrect();
        }
    } else {
        incorrect();
    }
}


fn incorrect() {
    println!("Sorry, that was incorrect!");
    println!("Better luck next time!");
}
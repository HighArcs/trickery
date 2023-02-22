use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "What is your name?");
    let name = s.next_line();

    writeln!(f, "What is your favourite number?");
    let n = s.next_i32();

    writeln!(f, "Your name is {} and you like the number {}.", name, n)
}

pub fn activity_two(mut f: impl std::io::Write) {
    let s = Scanner::new();

    let mut order = String::from("apple pie");

    writeln!(f, "The current order is {}", order);

    writeln!(f, "I want to eat something else, what do you want to eat?");
    order = s.next_line();

    writeln!(f, "The otder has changed to {}", order)
}

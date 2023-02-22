use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "What is your favourite food?");

    let content = s.next_line();

    writeln!(f, "I like to eat {} as well!", content);
}

pub fn activity_two(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "Please enter four names:");

    let a = s.next_line();
    let b = s.next_line();
    let c = s.next_line();
    let d = s.next_line();

    writeln!(f, "{} {} {} {}", d, b, c, a);
}

pub fn activity_three(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "Hi there. What is your name?");
    let name = s.next_line();

    writeln!(f, "What state do you live in?");
    let location = s.next_line();

    writeln!(f, "My name is {}. I live in {}.", name, location);
}

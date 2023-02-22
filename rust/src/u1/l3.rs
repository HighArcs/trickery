use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "Print 3 doubles:");

    let a = s.next_double();
    let b = s.next_double();
    let c = s.next_double();

    writeln!(f, "{} {} {}", c, b, a)
}

pub fn activity_two(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "Hi there. What is your name?");

    let name = s.next_line();

    writeln!(f, "Hi {}. How old are you?", name);
    let age = s.next_u8();

    writeln!(f, "{} is {} years old.", name, age)
}

pub fn activity_three(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "Java is an object-oriented programming language, true or false?");
    let u = s.next_bool();

    writeln!(f, 
        "There are only 2 possible values which can be held by a boolean variable, true or false?"
    );
    let p = s.next_bool();

    writeln!(f, "Question 1 - Your answer: {}. Correct answer: true", u);
    writeln!(f, "Question 2 - Your answer: {}. Correct answer: true", p);
}

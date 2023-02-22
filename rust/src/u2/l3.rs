use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write, ) {
    let s = Scanner::new();

    writeln!(f, "Enter a string:");
    let content = s.next_line();

    writeln!(f, "Enter a number:");
    let location = s.next_usize();

    writeln!(f, "{}{}", &content[0..location], &content[location..])
}

pub fn activity_two(mut f: impl std::io::Write, ) {
    let s = Scanner::new();

    writeln!(f, "Enter a string:");
    let content = s.next_line();

    writeln!(f, "How many characters would you like to delete at the end?");
    let count = s.next_usize();

    writeln!(f, "{}", &content[0..count]);
}

pub fn activity_three(mut f: impl std::io::Write, ) {
    let s = Scanner::new();

    writeln!(f, "Enter first word:");
    let a = s.next_line().to_lowercase();

    writeln!(f, "Enter second word:");
    let b = s.next_line().to_lowercase();

    writeln!(f, "Result: {:?}", a.cmp(&b))
}

pub fn activity_four() {
    let s = Scanner::new();

    writeln!(f, "Enter a sentence:");
    let content = s.next_line();

    let word = content.split_ascii_whitespace().next().unwrap();

    writeln!(f, "{}", word.len())
}

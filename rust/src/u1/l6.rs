use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    let n = s.next_i32();
    let d = s.next_i32();

    writeln!(f, "The decimal value is: {}", (n as f64) / (d as f64));
}

pub fn activity_two(mut f: impl std::io::Write) {
    let s = Scanner::new();

    let a = s.next_double().round() as i32;
    let b = s.next_double().round() as i32;

    writeln!(f, "Answer: {} + {} = {}", a, b, a + b);
}

pub fn activity_three(mut f: impl std::io::Write) {
    let s = Scanner::new();
    let d = s.next_double();

    let a = (d * 10.0 % 10.0) as i32;
    let b = (d * 100.0 % 10.0) as i32;
    let c = (d * 1000.0 % 10.0) as i32;

    writeln!(f, "Answer: {} {} {}", a, b, c);
}

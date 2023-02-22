use crate::tools::Scanner;

pub fn activity_one() {
    for i in (1..=25).step_by(2) {
        writeln!(f, "{i}")
    }
}

pub fn activity_two() {
    for i in 17..=73 {
        if i % 10 == 6 {
            writeln!(f, "{i}");
        } else {
            write!(f, "{i} ")
        }
    }
}

pub fn activity_three() {
    let s = Scanner::new();

    writeln!(f, "Enter a number between 0 and 50:");
    let n = s.next_i32();

    if n <= 0 || n >= 50 {
        writeln!(f, "error");
    } else {
        for i in n..=50 {
            if i % 5 == n % 5 {
                write!(f, "\n");
            }

            write!(f, "{i} ")
        }
    }
}

pub fn activity_four() {
    let s = Scanner::new();

    writeln!(f, "Enter a positive integer:");
    let n = s.next_i32();

    if n <= 0 {
        writeln!(f, "error");
    } else {
        for i in (0..n).rev().step_by(3) {
            if i % 3 == 0 {
                write!(f, "{i} ")
            }
        }
    }
}
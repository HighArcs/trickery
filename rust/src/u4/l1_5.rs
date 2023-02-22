use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    let n = s.next_i32();

    let mut fac = 1;
    while fac <= 5 {
        writeln!(f, "{}", n * fac);
        fac += 1;
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    let mut n = s.next_i32();

    let mut sum = 0;
    while n > 0 {
        sum += {
            n -= 1;
            n
        };
    }

    writeln!(f, "{sum}")
}

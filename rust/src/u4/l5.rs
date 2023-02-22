use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    writeln!(f, "Input a String:");
    let st = &*s.next_line();

    writeln!(f, "Input an integer");
    let n = s.next_i32();

    for i in 0..st.len() {
        for _ in 0..n {
            write!(f, "{}", &st[(i as usize)..(i as usize) + 1]);
        }
    }
}

pub fn activity_two() {
    for i in (0..=10).rev() {
        for _ in 0..i {
            write!(f, "{i} ");
        }

        writeln!(f, );
    }
}
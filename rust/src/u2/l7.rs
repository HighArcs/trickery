use crate::tools::Scanner;

pub fn activity_one() {
    let mut a = i32::MAX;
    a = a.wrapping_add(1);
    writeln!(f, "{a}");

    let mut b = i32::MIN;
    b = b.wrapping_sub(1);

    writeln!(f, "{b}");
}

pub fn activity_two() {
    let s = Scanner::new();
    let mut x = None;
    let mut y = None;

    writeln!(f, "{x:?} {y:?}");

    x = Some(s.next_i32());
    y = Some(s.next_i32());

    let avg = Some((x.unwrap() as f64 + y.unwrap() as f64) / 2.0);
    writeln!(f, "Average of {x:?} and {y:?} is {avg:?}")
}

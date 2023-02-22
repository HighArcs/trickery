use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    let mut arr = [0.0; 3];
    arr[0] = s.next_double();
    arr[1] = s.next_double();
    arr[2] = s.next_double();

    writeln!(f, "Contents: {} {} {}", arr[0], arr[1], arr[2]);
    writeln!(f, "Sum: {}", arr[0] + arr[1] + arr[2]);
}

pub fn activity_two() {
    let mut h = [0; 10];

    h[0] = 1;
    h[1] = h[0] + 2;
    h[2] = h[1] + 3;
    h[3] = h[2] + 4;
    h[4] = h[3] + 5;
    h[5] = h[4] + 6;
    h[6] = h[5] + 7;
    h[7] = h[6] + 8;
    h[8] = h[7] + 9;
    h[9] = h[8] + 10;

    let s = Scanner::new();

    let i = s.next_usize();
    if let 1..=10 = i {
        writeln!(f, "{}", h[i - 1])
    }
}


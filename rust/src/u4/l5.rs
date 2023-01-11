use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Input a String:");
    let st = &*s.next_line();

    println!("Input an integer");
    let n = s.next_i32();

    for i in 0..st.len() {
        for _ in 0..n {
            print!("{}", &st[(i as usize)..(i as usize) + 1]);
        }
    }
}

pub fn activity_two() {
    for i in (0..=10).rev() {
        for _ in 0..i {
            print!("{i} ");
        }

        println!();
    }
}
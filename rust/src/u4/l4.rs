use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Input String:");
    let st = s.next_line().to_lowercase();

    let mut count = 0;
    for i in 0..st.len() - 1 {
        let first = &st[i..i + 1];
        let second = &st[i + 1..i + 2];

        if first == "p"
            && (second == "a" || second == "e" || second == "i" || second == "o" || second == "u")
        {
            count += 1;
        }
    }

    println!("Contains p followed by a vowel {count} times.")
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Input String:");

    let st = s.next_line().to_lowercase();

    for i in 0..st.len() {
        let c = &st[i..i + 1];

        if c != "e" && c != "t" && c != "a" && c != "i" && c != "o" {
            print!("{c}");
        }
    }

    print!("\n");
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Enter two strings:");
    let a = &*s.next_line();
    let b = &*s.next_line();

    if a.len() != b.len() {
        println!("error");
    } else {
        for i in (0..=(a.len() - 1)).rev() {
            print!("{}", &b[i..i+1]);
            print!("{}", &a[i..i+1]);
        }
    }
}

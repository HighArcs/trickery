use std::str::FromStr;

use crate::tools::Scanner;

pub fn assignment4() {
    let s = Scanner::new();

    println!("Type the message to be shortened");
    let st = s.next_line().to_lowercase();

    let mut m1 = String::new();
    let mut vowels = 0;
    let mut repeat = 0;

    let mut m2 = String::new();
    let mut reached = String::from_str(" ").unwrap();

    for i in 0..st.len() {
        let c = &st[i..i + 1];

        for _ in 0..1 {
            if i == 0 {
                m1 += c;
                continue;
            }

            let p = &st[i - 1..i];

            if p == " " {
                m1 += c;
                continue;
            }

            match c {
                "a" | "e" | "i" | "o" | "u" => {
                    vowels += 1;
                    continue;
                }

                _ => {
                    if c == p {
                        repeat += 1;
                    } else {
                        m1 += c;
                    }
                    continue;
                }
            }
        }

        for _ in 0..1 {
            if reached.contains(c) {
                continue;
            }

            reached += c;

            let mut count = 0;
            for j in 0..st.len() {
                if &st[j..j + 1] == c {
                    count += 1;
                }
            }

            m2 += count.to_string().as_str();
            m2 += c;
        }
    }

    println!();
    println!("Algorithm 1");
    println!("Vowels removed: {vowels}");
    println!("Repeats removed: {repeat}");
    println!("Algorithm 1 message: {m1}");
    println!("Algorithm 1 characters saved: {}", st.len() - m1.len());

    println!();
    println!("Algorithm 2");
    println!("Unique characters found: {}", reached.len() - 1);
    println!("Algorithm 2 message: {m2}");
    println!("Algorithm 2 characters saved: {}", st.len() - m1.len())
}

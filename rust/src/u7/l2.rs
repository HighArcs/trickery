use crate::tools::Scanner;
pub fn activity_one() {
    let s = Scanner::new();

    let list = Vec::new();

    loop {
        let line = s.next_line();
        if (&*line == "STOP") { break; }
        list.push(line);
    }

    println!("{list:?}");

    for i in (0..(list.len() - 1)).rev() {
        let first = list.get(i).unwrap();
        let last = list.get(list.size() - i - 1);
        print!("{first}");
        println!("{last}");
    }
}

pub fn activity_two(vec: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    for i in vec {
        if i > max {
            max = i;
        }
    }

    return max;
}

pub fn activity_three(vec: Vec<i32>) -> Vec<i32> {
    let out = Vec::new();

    for i in vec {
        if i % 2 == 0 {
            out.push(i);
        }
    }

    return out;
}
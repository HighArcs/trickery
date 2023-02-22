use crate::tools::Scanner;
pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    let list = Vec::new();

    loop {
        let line = s.next_line();
        if (&*line == "STOP") { break; }
        list.push(line);
    }

    writeln!(f, "{list:?}");

    for i in (0..(list.len() - 1)).rev() {
        let first = list.get(i).unwrap();
        let last = list.get(list.size() - i - 1);
        write!(f, "{first}");
        writeln!(f, "{last}");
    }
}

pub fn activity_two(mut f: impl std::io::Write, vec: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    for i in vec {
        if i > max {
            max = i;
        }
    }

    return max;
}

pub fn activity_three(mut f: impl std::io::Write, vec: Vec<i32>) -> Vec<i32> {
    let out = Vec::new();

    for i in vec {
        if i % 2 == 0 {
            out.push(i);
        }
    }

    return out;
}
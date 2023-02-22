use crate::tools::Scanner;
pub fn activity_one() {
    let scanner = Scanner::new();

    let mut vec = Vec::new();
    writeln!(f, "Please enter words, enter STOP to stop the loop.");
    loop {
        let line = scanner.next_line();
        if &*line == "STOP" {
            break;
        }

        vec.push(line);
    }

    writeln!(f, "{}", vec.len());
    writeln!(f, "{vec:?}");

    if vec.len() > 2 {
        let temp = vec.get(0).unwrap();
        vec[0] = vec.last().unwrap();
        vec[vec.len() - 1] = temp;
        vec.remove(0);
    }

    writeln!(f, "{vec:?}");
}
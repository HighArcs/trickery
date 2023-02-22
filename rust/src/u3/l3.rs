use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    writeln!(f, "Please enter an integer");
    let n = s.next_i32();

    if n % 2 == 0 {
        writeln!(f, "Even");
    } else {
        writeln!(f, "Odd");
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    writeln!(f, "Please enter a letter grade:");
    let letter = s.next_line();

    writeln!(f, 
        "{}",
        match &*letter {
            "A" => "90-100",
            "B" => "80-90",
            "C" => "70-80",
            "D" => "60-70",
            "F" => "0-60",
            _ => "Invalid letter grade",
        }
    )
}

pub fn activity_three() {
    let s = Scanner::new();

    writeln!(f, "Please enter two integers:");
    let a = s.next_i32();
    let b = s.next_i32();

    writeln!(f, "{} + {} = ?", a, b);
    let given = s.next_i32();

    if given == (a + b) {
        writeln!(f, "Correct!");
    } else {
        writeln!(f, "Wrong");
    }
}

pub fn activity_four() {
    let s = Scanner::new();

    writeln!(f, "What is the temperature?");
    let temperature = s.next_i32();

    if temperature >= 97 {
        if temperature <= 99 {
            writeln!(f, "Temperature is OK");
        } else {
            writeln!(f, "NOT NORMAL");
        }
    } else {
        writeln!(f, "NOT NORMAL")
    }
}

use crate::tools::Scanner;

pub fn assignment3() {
    let s = Scanner::new();
    writeln!(f, "Welcome. what is your name?");
    let name = s.next_line();

    writeln!(f, "Hello {name}. Are you ready to crack the code?");
    let is_ready = s.next_line().to_lowercase() == "yes";

    if !is_ready {
        return;
    }

    writeln!(f, );
    writeln!(f, "PHASE 1");
    writeln!(f, "Enter a string: ");
    let content = s.next_line();

    if content.len() == 3 {
        writeln!(f, "Correct!");

        writeln!(f, );
        writeln!(f, "PHASE 2");
        writeln!(f, "Enter a number:");

        let n = s.next_i32();
        if n == 19 || (n >= 35 && n < 78) {
            writeln!(f, "Correct!");

            writeln!(f, );
            writeln!(f, "PHASE 3");
            writeln!(f, "Enter a number:");

            let u = s.next_i32();

            if u > 0 && (u % 2 == 0 || u % 10 == 1) {
                writeln!(f, "Correct!");
                writeln!(f, "You have cracked the code!");
            } else {
                incorrect();
            }
        } else {
            incorrect();
        }
    } else {
        incorrect();
    }
}


fn incorrect() {
    writeln!(f, "Sorry, that was incorrect!");
    writeln!(f, "Better luck next time!");
}
use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();

    writeln!(f, "What type of item are you buying?");
    let item = s.next_line();

    writeln!(f, "How many are you buying?");
    let count = s.next_i32();

    writeln!(f, "How much does each one weight");
    let weight = s.next_double();

    writeln!(f, 
        "{} {} at {} pounds each will weigh {} pounds total",
        count,
        item,
        weight,
        (count as f64) * weight
    )
}

pub fn activity_two(mut f: impl std::io::Write) {
    writeln!(f, 
        "{}{}{}",
        "\"That brain of mine is something more than merely mortal; ",
        "as time will show.\"\nAda Lovelace",
        "\nThe first computer programmer"
    )
}

pub fn activity_three(mut f: impl std::io::Write) {
  write!(f, "(\\(\\\n( - -)\n((') (')");
}
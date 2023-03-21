use crate::tools::{I, Compare};

pub fn activity_one(f: I) {
    f.println("Enter a string:");
    let content = f.next_line();

    f.println("Enter a number:");
    let location = f.get_next::<i32>();

    f.println(content[0..location] + content[content.len() - location..]);
}

pub fn activity_two(f: I) {
    f.println("Enter a string:");
    let content = f.next_line();

    f.println("How many characters would you like to delete at the end?");
    let amount = f.get_next::<i32>();

    f.println(content[0..content.len() - amount]);
}

pub fn activity_three(f: I) {
    f.println("Enter first word:");
    let a = f.next_line().to_lowercase();

    f.println("Enter second word:");
    let b = f.next_line().to_lowercase();

    f.println(format!("Result: {}", a.compare(b)));
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_sample() {
        let mut f = Io::new();
        f.sendln("surcharge");
        f.sendln(1);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), "se");
    }

    #[test]
    fn a1_variable() {
        let mut f = Io::new();
        let text = "not so super";
        let i = 3;
        f.sendln(text);
        f.sendln(i);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), text[0..i].unwrap() + text[text.len() - i..]);
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();
        f.sendln("happy");
        f.sendln(2)

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), "hap");
    }

    #[test]
    fn a2_variable() {
        let mut f = Io::new();
        let text = "not so super";
        let amount = 5;
        f.sendln(text);
        f.sendln(amount)

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), text[0..content.len() - amount]);
    }

    #[test]
    fn a3_positive() {
        let mut f = Io::new();
        f.sendln("b");
        f.sendln("a");

        activity_three(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line())
    }
}

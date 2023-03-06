use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("Print 3 doubles:");

    let a = f.next::<f64>();
    let b = f.next::<f64>();
    let c = f.next::<f64>();

    f.println(format!("{c} {b} {a}"));
}

pub fn activity_two(f: I) {
    f.println("Hi there. What is your name?");

    let name = f.next_line();

    f.println(format!("Hi {name}. How old are you?"));

    let years = f.next::<usize>();

    f.println(format!("{name} is {years} years old."));
}

pub fn activity_three(f: I) {
    f.println("Java is an object-oriented programming language, true or false?");
    let u = f.next::<bool>();

    f.println("Java is an object-oriented programming language, true or false?");
    let p = f.next::<bool>();

    f.println(format!(
        "Question 1 - Your answer: {u}. Correct answer: true"
    ));
    f.println(format!(
        "Question 2 - Your answer: {p}. Correct answer: true"
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_sample() {
        let mut f = Io::new();

        f.sendln("1.3");
        f.sendln("2.7");
        f.sendln("6.8");

        activity_one(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "6.8 2.7 1.3");
    }

    #[test]
    fn a1_variable() {
        let mut f = Io::new();

        let a = "3.4";
        let b = "6.2";
        let c = "9.3";

        f.sendln(a);
        f.sendln(b);
        f.sendln(c);

        activity_one(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), format!("{c} {b} {a}"));
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();

        f.sendln("Pascal");
        f.sendln("16");

        activity_two(&mut f);

        f.read_line();

        // assert_eq!(f.read_line(), "Pascal");
        assert_eq!(f.read_line(), "Hi Pascal. How old are you?");
        assert_eq!(f.read_line(), "Pascal is 16 years old.");
    }

    #[test]
    fn a2_variable() {
        let mut f = Io::new();

        let name = "Pascal";
        let age = "16";

        f.sendln(name);
        f.sendln(age);

        activity_two(&mut f);

        f.read_line();

        // assert_eq!(f.read_line(), name);
        assert_eq!(f.read_line(), format!("Hi {name}. How old are you?"));
        assert_eq!(f.read_line(), format!("{name} is {age} years old."));
    }

    #[test]
    fn a2_uses_age() {
        let mut f = Io::new();

        let age = "16";

        f.sendln("Pascal");
        f.sendln(age);

        activity_two(&mut f);

        f.read_line(); // consume

        // assert_eq!(f.read_line(), "Pascal");
        assert_eq!(f.read_line(), "Hi Pascal. How old are you?");
        assert_eq!(f.read_line(), format!("Pascal is {age} years old."));
    }

    #[test]
    fn a2_uses_inputs() {
        let mut f = Io::new();

        let name = "Pascal";
        let age = "16";

        f.sendln(name);
        f.sendln(age);

        activity_two(&mut f);

        f.read_line();

        // assert_eq!(f.read_line(), name);
        assert_eq!(f.read_line(), format!("Hi {name}. How old are you?"));
        assert_eq!(f.read_line(), format!("{name} is {age} years old."));
        assert!(f.i.is_empty())
    }

    #[test]
    fn a3_sample() {
        let mut f = Io::new();

        f.sendln("true");
        f.sendln("false");

        activity_three(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(
            f.read_line(),
            "Question 1 - Your answer: true. Correct answer: true"
        );
        assert_eq!(
            f.read_line(),
            "Question 2 - Your answer: false. Correct answer: true"
        );
    }

    #[test]
    fn a3_variable() {
        let mut f = Io::new();

        let u = "true";
        let p = "false";

        f.sendln(u);
        f.sendln(p);

        activity_three(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(
            f.read_line(),
            format!("Question 1 - Your answer: {u}. Correct answer: true")
        );
        assert_eq!(
            f.read_line(),
            format!("Question 2 - Your answer: {p}. Correct answer: true")
        );
    }
}

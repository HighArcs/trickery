use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("Enter your favourite food:");
    let food = f.next_line();

    f.println(format!("I like to eat {food} as well!"));
}

pub fn activity_two(f: I) {
    f.println("List four names:");

    let a = f.next_line();
    let b = f.next_line();
    let c = f.next_line();
    let d = f.next_line();

    f.println(format!("{d} {b} {c} {a}"));
}

pub fn activity_three(f: I) {
    f.println("Hi there. What is your name?");

    let name = f.next_line();

    f.println("What state do you live in?");

    let state = f.next_line();

    f.println(format!("My name is {name}. I live in {state}."));
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_name() {
        let mut f = Io::new();

        let value = "value";
        f.send(value);

        activity_one(&mut f);

        f.read_line(); // consume

        assert_eq!(f.read_line(), format!("I like to eat {value} as well!"));
    }

    #[test]
    fn a1_direct() {
        let mut f = Io::new();

        f.send("burritos");

        activity_one(&mut f);

        f.read_line(); // consume

        assert_eq!(f.read_line(), "I like to eat burritos as well!");
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();

        f.sendln("Jesse");
        f.sendln("David");
        f.sendln("Elaine");
        f.sendln("Sandy");

        activity_two(&mut f);

        f.read_line();
        assert_eq!(f.read_line(), "Sandy David Elaine Jesse");
    }

    #[test]
    fn a2_variable() {
        let mut f = Io::new();

        let a = "Jesse";
        let b = "David";
        let c = "Elaine";
        let d = "Sandy";

        f.sendln(a);
        f.sendln(b);
        f.sendln(c);
        f.sendln(d);

        activity_two(&mut f);

        f.read_line();
        assert_eq!(f.read_line(), format!("{d} {b} {c} {a}"));
    }

    #[test]
    fn a3_sample() {
        let mut f = Io::new();

        f.sendln("Dave");

        f.sendln("New York");

        activity_three(&mut f);

        assert_eq!(f.read_line(), "Hi there. What is your name?");
        assert_eq!(f.read_line(), "What state do you live in?");
        assert_eq!(f.read_line(), "My name is Dave. I live in New York.");
    }

    #[test]
    fn a3_variable() {
        let mut f = Io::new();

        let name = "John";
        let state = "Maine";

        f.sendln(name);

        f.sendln(state);

        activity_three(&mut f);

        assert_eq!(f.read_line(), "Hi there. What is your name?");
        assert_eq!(f.read_line(), "What state do you live in?");
        assert_eq!(
            f.read_line(),
            format!("My name is {name}. I live in {state}.")
        );
    }
}

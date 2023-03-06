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

    f.print(format!("{d} {b} {c} {a}"));
}

#[cfg(test)]
mod tests {
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
}

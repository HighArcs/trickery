use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("What type of item are you buying?");
    let item = f.next_line();

    f.println("How many are you buying?");
    let count = f.get_next::<u32>();

    f.println("How much does one weigh?");
    let weight = f.get_next::<f64>();

    f.println(format!(
        "{count} {item} at {weight} pounds each will weigh {} pounds total",
        count.into() * weight
    ))
}

pub fn activity_two(f: I) {
    f.println("\"That brain of mine is something more than merely mortal; as time will show.\"\nAda Lovelace\nThe first computer programmer");
}

pub fn activity_three(f: I) {
    f.println("(\\(\\\n( - -)\n((') (')");
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_sample() {
        let mut f = Io::new();
        f.sendln("widgets");
        f.sendln("11");
        f.sendln("3.75");

        activity_one(&mut f);

        f.read_line();
        f.read_line();
        f.read_line();

        assert_eq!(
            f.read_line(),
            "11 widgets at 3.75 pounds each will weigh 41.25 pounds total"
        );
    }

    #[test]
    fn a1_variable() {
        let mut f = Io::new();
        let (name, count, weight) = ("telephone", 32, 1.63);
        f.sendln(name);
        f.sendln(count);
        f.sendln(weight);

        activity_one(&mut f);

        f.read_line();
        f.read_line();
        f.read_line();

        assert_eq!(
            f.read_line(),
            format!(
                "{count} {name} at {weight} pounds each will weigh {} pounds total",
                count.into() * weight
            )
        );
    }

    #[test]
    fn a2_text() {
        let mut f = Io::new();

        activity_two(&mut f);

        assert_eq!(
            f.read_line(),
            "\"That brain of mine is something more than merely mortal; as time will show.\""
        );
        assert_eq!(f.read_line(), "Ada Lovelace");
        assert_eq!(f.read_line(), "The first computer programmer");
    }

    #[test]
    fn a3_text() {
        let mut f = Io::new();

        activity_three(&mut f);

        assert_eq!(f.read_line(), "(\\(\\");
        assert_eq!(f.read_line(), "( - -)");
        assert_eq!(f.read_line(), "((') (')");
    }
}

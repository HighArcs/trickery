use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("What is your name?");
    let name = f.next_line();

    f.println("What is your favorite number?");
    let n = f.get_next::<i32>();

    f.println(format!("Your name is {name} and you like the number {n}."));
}

pub fn activity_two(f: I) {
    let mut order = "apple pie";
    f.println(format!("The current order is {order}"));
    f.println("I want to eat something else, what do you want to eat?");
    order = &f.next_line()[..];
    f.println(format!("The order has changed to {order}"));
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_sample() {
        let mut f = Io::new();
        f.sendln("Cory");
        f.sendln("48");

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), "Your name is Cory and you like the number 48.");
    }

    #[test]
    fn a1_name() {
        let mut f = Io::new();
        let name = "abc"
        f.sendln(name);
        f.sendln("48");

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), format!("Your name is {name} and you like the number 48."));
    }

    #[test]
    fn a1_number() {
        let mut f = Io::new();
        let number = "25"
        f.sendln("Evan");
        f.sendln(number);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), format!("Your name is Evan and you like the number {number}."));
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();
        f.sendln("burger");

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), "The order has changed to burger");
    }

    #[test]
    fn a2_variable() {
        let mut f = Io::new();
        let order = "pizza";
        f.sendln(order);

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), format!("The order has changed to {order}"));
    }

    
}

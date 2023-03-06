use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("Enter starting number (must be an integer)");
    let mut i = f.next::<i32>();

    i += 1;
    f.println(format!("number is now {i}"));
    i += 1;
    f.println(format!("number is now {i}"));
    i += 1;
    f.println(format!("number is now {i}"));
    i += 1;
    f.println(format!("number is now {i}"));
    i -= 1;
    f.println(format!("number is now {i}"));
    i -= 1;
    f.println(format!("number is now {i}"));
    i -= 1;
    f.println(format!("number is now {i}"));
    i -= 1;
    f.println(format!("number is now {i}"));
}

pub fn activity_two(f: I) {
    let i = f.next::<i32>();

    f.println(i / 3);
}

pub fn activity_three(f: I) {
    f.println("Enter a circumference:");

    let circumference = f.next::<f64>();

    let radius = circumference / 6.28;
    let area = radius * radius * 3.14;

    f.println(format!("Radius: {radius}"));
    f.println(format!("Area: {area}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_sample() {
        let mut f = Io::new();

        f.sendln("24");

        activity_one(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "number is now 25");
        assert_eq!(f.read_line(), "number is now 26");
        assert_eq!(f.read_line(), "number is now 27");
        assert_eq!(f.read_line(), "number is now 28");
        assert_eq!(f.read_line(), "number is now 27");
        assert_eq!(f.read_line(), "number is now 26");
        assert_eq!(f.read_line(), "number is now 25");
        assert_eq!(f.read_line(), "number is now 24");
    }

    #[test]
    fn a1_variable() {
        let mut f = Io::new();
        let n = 27;
        f.sendln(n);

        activity_one(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), format!("number is now {}", n + 1));
        assert_eq!(f.read_line(), format!("number is now {}", n + 2));
        assert_eq!(f.read_line(), format!("number is now {}", n + 3));
        assert_eq!(f.read_line(), format!("number is now {}", n + 4));
        assert_eq!(f.read_line(), format!("number is now {}", n + 3));
        assert_eq!(f.read_line(), format!("number is now {}", n + 2));
        assert_eq!(f.read_line(), format!("number is now {}", n + 1));
        assert_eq!(f.read_line(), format!("number is now {}", n + 0));
    }

    #[test]
    fn a1_count() {
        let mut f = Io::new();
        let n = 27;
        f.sendln(n);

        activity_one(&mut f);

        f.read_line();

        let mut count = 0;

        while f.read_line().starts_with("number is now") {
            count += 1;
        }

        assert_eq!(count, 8);
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();

        f.sendln("100");

        activity_two(&mut f);

        assert_eq!(f.read_line(), "33");
    }

    #[test]
    fn a2_other() {
        let mut f = Io::new();

        f.sendln("33");

        activity_two(&mut f);

        assert_eq!(f.read_line(), "11");
    }

    #[test]
    fn a2_variable() {
        let mut f = Io::new();

        let n = 45;

        f.sendln(n);

        activity_two(&mut f);

        assert_eq!(f.read::<i32>(), n / 3);
    }

    #[test]
    fn a3_sample() {
        let mut f = Io::new();

        f.sendln(25.12);

        activity_three(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Radius: 4.0");
        assert_eq!(f.read_line(), "Area: 50.24");
    }
}

use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("Please enter the numerator:");

    let n = f.get_next::<i32>();

    f.println("Please enter the denominator:");

    let d = f.get_next::<i32>();

    f.println(format!("The decimal value is: {}", (n as f64) / (d as f64)));
}

pub fn activity_two(f: I) {
    f.println("Please input two decimal numbers:");

    let a = f.get_next::<f64>();
    let b = f.get_next::<f64>();

    f.println(format!(
        "Answer: {} + {} = {}",
        a.round(),
        b.round(),
        (a.round() + b.round())
    ));
}

pub fn activity_three(f: I) {
    f.println("Please input a decimal number:");

    let d = f.get_next::<f64>();

    let a = (d *   10.0 % 10.0) as u8;
    let b = (d *  100.0 % 10.0) as u8;
    let c = (d * 1000.0 % 10.0) as u8;

    f.println(format!("Answer: {a} {b} {c}"))
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::Io;

    #[test]
    fn a1_sample() {
        let mut f = Io::new();

        f.sendln("5");
        f.sendln("20");

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), "The decimal value is: 0.25")
    }

    #[test]
    fn a1_variable_integer() {
        let mut f = Io::new();
        let (n, d) = (20, 5);

        f.sendln(n);
        f.sendln(d);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read_line(), format!("The decimal value is: {}", n / d))
    }

    #[test]
    fn a1_variable_decimal() {
        let mut f = Io::new();
        let (n, d) = (1, 5);

        f.sendln(n);
        f.sendln(d);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(
            f.read_line(),
            format!("The decimal value is: {}", n as f64 / d as f64)
        )
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();

        f.sendln("57.3934");
        f.sendln("22.5211");

        activity_two(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 57 + 23 = 80");
    }

    #[test]
    fn a2_round_down() {
        let mut f = Io::new();

        f.sendln("1.1");
        f.sendln("2");

        activity_two(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 1 + 2 = 3");
    }

    #[test]
    fn a2_round_three() {
        let mut f = Io::new();

        f.sendln("1.8");
        f.sendln("2");

        activity_two(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 2 + 2 = 4");
    }

    #[test]
    fn a3_sample() {
        let mut f = Io::new();

        f.sendln("67.3424");

        activity_three(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 3 4 2");
    }

    #[test]
    fn a3_other_sample() {
        let mut f = Io::new();

        f.sendln("12.054");

        activity_three(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 0 5 4");
    }

    #[test]
    fn a3_variable() {
        let mut f = Io::new();

        let n = 14.585;

        f.sendln(n);

        activity_three(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), format!("Answer: {} {} {}", (n * 10 % 10) as u8, (n * 100 % 10) as u8, (n * 1000 % 10) as u8));
    }

    #[test]
    fn a3_zeroes() {
        let mut f = Io::new();

        f.sendln("1.000");

        activity_three(&mut f);

        f.read_line();

        assert_eq!(f.read_line(), "Answer: 0 0 0");
    }
}

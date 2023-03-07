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
}

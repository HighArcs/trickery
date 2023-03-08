use crate::tools::I;

pub fn activity_one(f: I) {
    f.println("Please enter a three digit number:");

    let i = f.get_next::<i32>();

    assert!((i < 1000) & (i > 99));

    f.println("Here are the digits:");

    f.println(i / 100);
    f.println(i / 10 % 10);
    f.println(i % 10);
}

pub fn activity_two(f: I) {
    f.println("Please enter a four digit number:");

    let i: i32 = f.get_next();

    assert!((i < 10000) & (i > 999));

    f.println("Here are the digits:");

    f.println(i % 10);
    f.println(i / 10 % 10);
    f.println(i / 100 % 10);
    f.println(i / 1000 % 10);
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;

    #[test]
    fn a1_sample() {
        let mut f = Io::new();

        f.sendln(678);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), 6);
        assert_eq!(f.read::<i32>(), 7);
        assert_eq!(f.read::<i32>(), 8);
    }

    #[test]
    fn a1_sample_no_extras() {
        let mut f = Io::new();

        f.sendln(678);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), 6);
        assert_eq!(f.read::<i32>(), 7);
        assert_eq!(f.read::<i32>(), 8);
        assert_eq!(f.read_line(), "");
    }

    #[test]
    fn a1_variable_no_extras() {
        let mut f = Io::new();

        let n = 245;

        f.sendln(n);

        activity_one(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), n / 100);
        assert_eq!(f.read::<i32>(), n / 10 % 10);
        assert_eq!(f.read::<i32>(), n % 10);
        assert_eq!(f.read_line(), "");
    }

    #[test]
    fn a2_sample() {
        let mut f = Io::new();

        f.sendln(5678);

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), 8);
        assert_eq!(f.read::<i32>(), 7);
        assert_eq!(f.read::<i32>(), 6);
        assert_eq!(f.read::<i32>(), 5);
    }

    #[test]
    fn a2_sample_no_extras() {
        let mut f = Io::new();

        f.sendln(5678);

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), 8);
        assert_eq!(f.read::<i32>(), 7);
        assert_eq!(f.read::<i32>(), 6);
        assert_eq!(f.read::<i32>(), 5);
    }

    #[test]
    fn a2_variable_no_extras() {
        let mut f = Io::new();

        let n = 8362;

        f.sendln(n);

        activity_two(&mut f);

        f.read_line();
        f.read_line();

        assert_eq!(f.read::<i32>(), n % 10);
        assert_eq!(f.read::<i32>(), n / 10 % 10);
        assert_eq!(f.read::<i32>(), n / 100 % 10);
        assert_eq!(f.read::<i32>(), n / 1000 % 10);
    }
}

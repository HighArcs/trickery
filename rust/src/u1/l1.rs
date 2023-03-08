use crate::tools::I;

pub fn activity_one(f: I) {
    f.println(" Stephen Curry ");
}

pub fn activity_two(f: I) {
    f.println(" Coding is fun! ");
    f.println(" Coding is fun! ");
    f.println(" Coding is fun! ");
}

pub fn activity_three(f: I) {
    f.println("=========");
    f.println("= 0   0 =   Rust is Awesome!");
    f.println("=   v   =");
    f.println("=========");
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;
    #[test]
    fn a1_name() {
        let mut f = Io::new();

        activity_one(&mut f);
        assert!(f.read_line().contains("Stephen Curry"))
    }

    #[test]
    fn a1_centered() {
        let mut f = Io::new();

        activity_one(&mut f);
        assert_eq!(f.read_line(), " Stephen Curry ")
    }

    #[test]
    fn a2_print_three_times() {
        let mut f = Io::new();

        activity_two(&mut f);
        assert!(f.read_line().contains("Coding is fun!"));
        assert!(f.read_line().contains("Coding is fun!"));
        assert!(f.read_line().contains("Coding is fun!"));
    }

    #[test]
    fn a2_centered() {
        let mut f = Io::new();

        activity_two(&mut f);
        assert_eq!(f.read_line(), " Coding is fun! ");
        assert_eq!(f.read_line(), " Coding is fun! ");
        assert_eq!(f.read_line(), " Coding is fun! ");
    }

    #[test]
    fn a3_text() {
        let mut f = Io::new();

        activity_three(&mut f);

        assert_eq!(f.read_line(), "=========");
        assert_eq!(f.read_line(), "= 0   0 =   Rust is Awesome!");
        assert_eq!(f.read_line(), "=   v   =");
        assert_eq!(f.read_line(), "=========");
    }
}

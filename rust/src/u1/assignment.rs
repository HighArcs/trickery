use crate::tools::I;

pub fn assignment(f: I) {
    f.println("Please enter the course name.");

    let name = f.next_line();

    f.println("Please enter the average time spent in a week for this course in minutes.");

    let mut time = f.get_next::<u32>();

    f.println("Please enter the homework grades for this course.");

    let ha = f.get_next::<u8>();
    let hb = f.get_next::<u8>();
    let hc = f.get_next::<u8>();
    let hd = f.get_next::<u8>();

    f.println("Please enter the quiz grades for this course.");

    let qa = f.get_next::<f64>();
    let qb = f.get_next::<f64>();

    f.println("Please enter the final exam grade for this course.");

    let e = f.get_next::<f64>();

    f.println(format!("Course name: {name}"));

    let minutes = time % 60;

    let mut hours = 0;

    while time > 60 {
        time -= 60;
        hours += 1;
    }

    let hwa = (ha + hb + hc + hd) / 4;
    let qwa = (qa + qb) / 2;

    f.println(format!("Weekly time spent: {hours}h{minutes}"));

    f.println(format!("Average homework grade: {hwa}"));
    f.println(format!("Average quiz grade: {qwa}"));
    f.println(format!("Final exam grade: {e}"));

    let hw = 0.35;
    let qw = 0.15;
    let fw = 0.50;

    let avg = ((hw * hwa) + (qw * qwa) + (e * fw));

    f.println(format!("Overall grade: {}", avg.round()));
}

#[cfg(test)]
mod t {
    use super::*;
    use crate::tools::*;

    #[test]
    fn sample() {
        let mut f = Io::new();

        f.sendln("AP Computer Science A");
        f.sendln("135");
        f.sendln("75");
        f.sendln("99");
        f.sendln("80");
        f.sendln("100");
        f.sendln("89.2");
        f.sendln("98.1");
        f.sendln("87.58");

        assignment(&mut f);

        f.read_line();
        f.read_line();
        f.read_line();
        f.read_line();
        f.read_line();
        
        assert_eq!(f.read_line(), "Course name: AP Computer Science A");
        assert_eq!(f.read_line(), "Weekly time spent: 2h15");
        assert_eq!(f.read_line(), "Average homework grade: 88.5");
        assert_eq!(f.read_line(), "Average quiz grade: 93.65");
        assert_eq!(f.read_line(), "Final exam grade: 87.58");
        assert_eq!(f.read_line(), "Overall grade: 89");
    }
}

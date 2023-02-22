use crate::{
    shapes::{circle::Circle, rectangle::Rectangle, regular_polygon::RegularPolygon},
    tools::Scanner,
};

pub fn activity_one() {
    let a = Circle::new(10.1);
    let b = Circle::new(14.0);
    let c = Circle::new(20.5);

    writeln!(f, "{a}");
    writeln!(f, "{b}");
    writeln!(f, "{c}");
}

pub fn activity_two() {
    let s = Scanner::new();

    writeln!(f, "Type a number for length and width:");
    let lw = s.next_double();

    writeln!(f, "Type a length:");
    let l = s.next_double();

    writeln!(f, "Type a width:");
    let w = s.next_double();

    writeln!(f, "{}", Rectangle::square(lw));
    writeln!(f, "{}", Rectangle::new(l, w));
}

pub fn activity_three() {
    let s = Scanner::new();

    writeln!(f, "Type a side length:");
    let l = s.next_double();

    writeln!(f, "{}", RegularPolygon::new(3, l));
    writeln!(f, "{}", RegularPolygon::new(4, l));
}

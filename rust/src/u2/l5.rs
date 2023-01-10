use crate::{
    shapes::{circle::Circle, rectangle::Rectangle, regular_polygon::RegularPolygon},
    tools::Scanner,
};

pub fn activity_one() {
    let a = Circle::new(10.1);
    let b = Circle::new(14.0);
    let c = Circle::new(20.5);

    println!("{a}");
    println!("{b}");
    println!("{c}");
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Type a number for length and width:");
    let lw = s.next_double();

    println!("Type a length:");
    let l = s.next_double();

    println!("Type a width:");
    let w = s.next_double();

    println!("{}", Rectangle::square(lw));
    println!("{}", Rectangle::new(l, w));
}

pub fn activity_three() {
    let s = Scanner::new();

    println!("Type a side length:");
    let l = s.next_double();

    println!("{}", RegularPolygon::new(3, l));
    println!("{}", RegularPolygon::new(4, l));
}

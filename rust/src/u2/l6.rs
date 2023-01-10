use crate::{
    shapes::{circle::Circle, regular_polygon::RegularPolygon},
    tools::Scanner,
};

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter the radius of the circle:");
    let radius = s.next_double();

    let circle = Circle::new(radius);
    let circumference = circle.get_circumference();
    let area = circle.get_area();

    println!(
        "A circle with a radius {} has a circumference of {} and an area of {}",
        radius, circumference, area
    )
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter number of sides:");
    let sides = s.next_u32();

    println!("Enter the side length:");
    let length = s.next_double();

    let mut polygon = RegularPolygon::new(sides, length);

    println!(
        "Area with {} sides: {}",
        polygon.get_num_sides(),
        polygon.get_area()
    );
    println!("Incrementing the number of sides by two");
    polygon.add_sides(2);

    println!(
        "Area with {} sides: {}",
        polygon.get_num_sides(),
        polygon.get_area()
    );
}

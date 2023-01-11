use crate::shapes::{regular_polygon::RegularPolygon, circle::Circle};

pub fn activity_one(polygon: RegularPolygon, polygon2: RegularPolygon) -> u32 {
    polygon.get_num_sides() + polygon2.get_num_sides()
}

pub fn activity_two(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x2 - x1) as f64).powi(2) + ((y2 - y1) as f64).powi(2)).sqrt()
}

pub fn activity_three(c1: Circle, c2: Circle) -> f64 {
    (c1.get_circumference() - c2.get_circumference()).abs()
}
use crate::shapes::{rectangle::Rectangle, regular_polygon::RegularPolygon};

pub fn activity_one(mut poly: RegularPolygon) {
    poly.set_num_sides(3);
}

pub fn activity_two(rec: Rectangle) {
    {
        rec
    };
    unimplemented!()
}

pub fn activity_three(mut polygon: RegularPolygon, sides: u32) {
    polygon.set_num_sides(sides)
}

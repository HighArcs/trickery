use crate::{tools::Scanner, shapes::regular_polygon::RegularPolygon};

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter password:");
    let input = s.next_line();

    if &*input == "bulbasaur" {
        println!("Access granted!");
    } else {
        println!("Access denied!")
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter 2 strings:");

    let a = s.next_line();
    let b = s.next_line();

    if a == b {
        println!("Equal!");
    } else if a.to_lowercase() == b.to_lowercase() {
        println!("Different case");
    } else if a.len() == b.len() && a[0..a.len() - 1] == b[0..b.len() - 1] {
        println!("Close enough");
    } else {
        println!("Try again!")
    }
}

pub fn activity_three() {
  let s = Scanner::new();

  println!("Enter the side length of the first regular polygon:");
  let side_length = s.next_double();

  println!("Enter the number of sides of the second regular polygon:");
  let side_count = s.next_u32();

  println!("Enter the side length of the second polygon:");
  let target_side_length = s.next_double();

  let source = RegularPolygon::of_length(side_length);
  let target = RegularPolygon::new(side_count, target_side_length);

  if source == target {
    println!("Congruent Regular Polygons!");
  } else {
    println!("Different Regular Polygons!")
  }
}

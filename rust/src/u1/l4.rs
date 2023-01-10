use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();
    let mut i = s.next_i32();

    i += 1;
    println!("number is now {}", i);
    i += 1;
    println!("number is now {}", i);
    i += 1;
    println!("number is now {}", i);
    i += 1;
    println!("number is now {}", i);
    i -= 1;
    println!("number is now {}", i);
    i -= 1;
    println!("number is now {}", i);
    i -= 1;
    println!("number is now {}", i);
    i -= 1;
    println!("number is now {}", i);
}

pub fn activity_two() {
  let s = Scanner::new();
  let i = s.next_i32();

  println!("number is now {}", i / 3);
}

pub fn activity_three() {
  let s = Scanner::new();
  let d = s.next_double();

  const PI: f64 = 3.14;
  const TAU: f64 = 6.28;

  let r = d / TAU;

  println!("Radius: {}", r);
  println!("Area: {}", PI * r * r)
}

pub fn activity_four() {
  let s = Scanner::new();
  let d = s.next_double();

  println!("Change from 10: ${}", 10.0 - d)
}

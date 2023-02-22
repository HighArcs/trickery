use crate::tools::Scanner;

pub fn activity_one() {
  let s = Scanner::new();

  writeln!(f, "Enter 2 integers:");
  let x = s.next_i32();
  let y= s.next_i32();

  let ratio = x as f64 / y as f64;
  if ratio > 1.0 && ratio <= 8.0 {
    writeln!(f, "Ratio OK")
  }
}

pub fn activity_two() {
  let s = Scanner::new();

  writeln!(f, "Enter two numbers:");

  let a = s.next_i32();
  let b = s.next_i32();

  if b != 0 && a % b == 0 {
    writeln!(f, "{b} is a factor of {a}");
  } else {
    writeln!(f, "{b} is not a factor of {a}");
  }
}
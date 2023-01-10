use crate::tools::Scanner;

pub fn activity_one() {
  let s = Scanner::new();

  let i = s.next_i32();

  let a = i / 100;
  let b = i / 10 % 10;
  let c = i % 10;

  println!("{}", a);
  println!("{}", b);
  println!("{}", c);
}

pub fn activity_two() {
  let s = Scanner::new();

  let i = s.next_i32();

  let a = i / 1000 % 10;
  let b = i / 100 % 10;
  let c = i / 10 % 10;
  let d = i % 10;

  println!("{}", d);
  println!("{}", c);
  println!("{}", b);
  println!("{}", a);
}

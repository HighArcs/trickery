use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
  let s = Scanner::new();

  let i = s.next_i32();

  let a = i / 100;
  let b = i / 10 % 10;
  let c = i % 10;

  writeln!(f, "{}", a);
  writeln!(f, "{}", b);
  writeln!(f, "{}", c);
}

pub fn activity_two(mut f: impl std::io::Write) {
  let s = Scanner::new();

  let i = s.next_i32();

  let a = i / 1000 % 10;
  let b = i / 100 % 10;
  let c = i / 10 % 10;
  let d = i % 10;

  writeln!(f, "{}", d);
  writeln!(f, "{}", c);
  writeln!(f, "{}", b);
  writeln!(f, "{}", a);
}

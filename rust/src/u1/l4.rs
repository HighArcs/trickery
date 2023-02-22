use crate::tools::Scanner;

pub fn activity_one(mut f: impl std::io::Write) {
    let s = Scanner::new();
    let mut i = s.next_i32();

    i += 1;
    writeln!(f, "number is now {}", i);
    i += 1;
    writeln!(f, "number is now {}", i);
    i += 1;
    writeln!(f, "number is now {}", i);
    i += 1;
    writeln!(f, "number is now {}", i);
    i -= 1;
    writeln!(f, "number is now {}", i);
    i -= 1;
    writeln!(f, "number is now {}", i);
    i -= 1;
    writeln!(f, "number is now {}", i);
    i -= 1;
    writeln!(f, "number is now {}", i);
}

pub fn activity_two(mut f: impl std::io::Write) {
  let s = Scanner::new();
  let i = s.next_i32();

  writeln!(f, "number is now {}", i / 3);
}

pub fn activity_three(mut f: impl std::io::Write) {
  let s = Scanner::new();
  let d = s.next_double();

  const PI: f64 = 3.14;
  const TAU: f64 = 6.28;

  let r = d / TAU;

  writeln!(f, "Radius: {}", r);
  writeln!(f, "Area: {}", PI * r * r)
}

pub fn activity_four(mut f: impl std::io::Write, ) {
  let s = Scanner::new();
  let d = s.next_double();

  writeln!(f, "Change from 10: ${}", 10.0 - d)
}

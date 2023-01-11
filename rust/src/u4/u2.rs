use crate::tools::Scanner;

pub fn activity_one() {
    let s = Scanner::new();

    println!("Enter two numbers:");
    let a = s.next_i32();
    let b = s.next_i32();

    let mut i = a;
    while i <= b {
        if i % 2 == 1 {
            print!("{i} ");
        }

        i += 1;
    }
}

pub fn activity_two() {
    let s = Scanner::new();

    println!("Enter a positive integer:");
    let mut n = s.next_i32();

    if n < 0 {
        println!("Not a positive integer")
    } else {
        let mut p = 0;
        while n > 0 {
            println!(
                "{}",
                (n % 10)
                    * 10i32.pow({
                        p += 1;
                        p - 1
                    })
            );
        }

        n /= 10;
    }

    {
        n
    };
}

pub fn activity_three() {
    let s = Scanner::new();

    let mut ok = true;

    let mut lo = 0.0;
    let mut la = 0.0;

    let mut mla = -90.0;
    let mut nla = 90.0;
    let mut mlo = -180.0; // milo :D
    let mut nlo = 180.0;

    while ok {
        println!("Please enter the longitude:");
        lo = s.next_double();

        println!("Please enter the latitude:");
        la = s.next_double();

        if la < -90.0 || la > 90.0 || lo < -180.0 || lo > 180.0 {
            println!("Incorrect Latitude or Longitude");
        } else {
            if la > mla {
                mla = la;
            }

            if la < nla {
                nla = la;
            }

            if lo > mlo {
                mlo = lo;
            }

            if lo < nlo {
                nlo = lo;
            }
        }

        println!("Would you like to enter another location?");
        ok = s.next_i32() == 1;
    }

    println!("Farthest North: {mla}");
    println!("Farthest South: {nla}");
    println!("Farthest East: {mlo}");
    println!("Farthest West: {nlo}");

    {
        la
    };
    {
        lo
    };
}

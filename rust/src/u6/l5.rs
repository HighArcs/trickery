pub fn activity_one<const T: usize>(words: [String; T]) {
    for s in words {
        for i in (0..s.len() - 1).rev() {
            print!("{}", &s[i..i + 1]);
        }

        println!();
    }
}

pub fn activity_two<const T: usize>(arr: [i32; T]) -> i32 {
    let mut p = 1;
    for k in arr {
        p *= k;
    }

    p
}

pub fn activity_three<const T: usize>(arr: [i32; T]) -> f64 {
    let mut s = 0;
    for n in arr {
        s += n;
    }

    return s as f64 / arr.len() as f64;
}